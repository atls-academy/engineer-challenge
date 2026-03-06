package app

import (
	"context"
	"crypto/sha256"
	"encoding/hex"
	"errors"
	"time"

	"github.com/atrump/engineer-challenge/internal/domain"
	"github.com/atrump/engineer-challenge/internal/pkg"
	"github.com/atrump/engineer-challenge/internal/pkg/rabbitmq"
	"github.com/cenkalti/backoff/v4"
	"github.com/google/uuid"
	"go.uber.org/zap"
)

var (
	ErrUserAlreadyExists  = errors.New("user already exists")
	ErrInvalidCredentials = errors.New("invalid credentials")
	ErrInvalidToken       = errors.New("invalid or expired token")
	ErrRateLimitExceeded  = errors.New("rate limit exceeded, please try again later")
)

// AuthCommandService is responsible for commands that change system state.
type AuthCommandService struct {
	readRepo     domain.UserReadRepository
	writeRepo    domain.UserWriteRepository
	tokenManager domain.TokenManager
	publisher    *rabbitmq.Publisher
	limiter      domain.RateLimiter
}

// AuthQueryService is responsible for queries that read system state.
type AuthQueryService struct {
	readRepo      domain.UserReadRepository
	tokenManager  domain.TokenManager
	sessionRepo   domain.SessionRepository
	refreshExpiry time.Duration
	limiter       domain.RateLimiter
}

func NewAuthCommandService(repo domain.UserRepository, tm domain.TokenManager, pub *rabbitmq.Publisher, limiter domain.RateLimiter) *AuthCommandService {
	return &AuthCommandService{
		readRepo:     repo,
		writeRepo:    repo,
		tokenManager: tm,
		publisher:    pub,
		limiter:      limiter,
	}
}

func NewAuthQueryService(readRepo domain.UserReadRepository, tm domain.TokenManager, sessionRepo domain.SessionRepository, refreshExpiry time.Duration, limiter domain.RateLimiter) *AuthQueryService {
	return &AuthQueryService{
		readRepo:      readRepo,
		tokenManager:  tm,
		sessionRepo:   sessionRepo,
		refreshExpiry: refreshExpiry,
		limiter:       limiter,
	}
}

type RegisterCommand struct {
	Email    string
	Password string
}

func (s *AuthCommandService) Register(ctx context.Context, cmd RegisterCommand) (domain.UserID, error) {
	allowed, err := s.limiter.Allow(ctx, "register:"+cmd.Email, 3, 10*time.Minute)
	if err != nil {
		return domain.UserID{}, err
	}
	if !allowed {
		return domain.UserID{}, ErrRateLimitExceeded
	}

	if err := domain.ValidatePasswordComplexity(cmd.Password); err != nil {
		return domain.UserID{}, err
	}

	email, err := domain.NewEmail(cmd.Email)
	if err != nil {
		return domain.UserID{}, err
	}

	existingUser, _ := s.readRepo.FindByEmail(ctx, email)
	if existingUser != nil {
		return domain.UserID{}, ErrUserAlreadyExists
	}

	hash, err := domain.HashPassword(cmd.Password)
	if err != nil {
		return domain.UserID{}, err
	}

	user := domain.NewUser(email, hash)
	if err := s.writeRepo.Save(ctx, user); err != nil {
		return domain.UserID{}, err
	}

	return user.ID, nil
}

type LoginQuery struct {
	Email    string
	Password string
}

func (s *AuthQueryService) Login(ctx context.Context, query LoginQuery) (string, string, time.Time, error) {
	allowed, err := s.limiter.Allow(ctx, "login:"+query.Email, 5, 5*time.Minute)
	if err != nil {
		return "", "", time.Time{}, err
	}
	if !allowed {
		return "", "", time.Time{}, ErrRateLimitExceeded
	}

	email, err := domain.NewEmail(query.Email)
	if err != nil {
		return "", "", time.Time{}, err
	}

	user, err := s.readRepo.FindByEmail(ctx, email)
	if err != nil || user == nil {
		return "", "", time.Time{}, ErrInvalidCredentials
	}

	if !domain.CheckPasswordHash(query.Password, user.PasswordHash) {
		return "", "", time.Time{}, ErrInvalidCredentials
	}

	at, rt, exp, err := s.tokenManager.GeneratePair(user.ID)
	if err != nil {
		return "", "", time.Time{}, err
	}

	session := &domain.Session{
		Token:     rt,
		UserID:    user.ID,
		ExpiresAt: time.Now().Add(s.refreshExpiry),
	}

	if err := s.sessionRepo.Save(ctx, session); err != nil {
		return "", "", time.Time{}, err
	}

	return at, rt, exp, nil
}

func (s *AuthCommandService) InitiatePasswordReset(ctx context.Context, emailStr string) error {
	allowed, err := s.limiter.Allow(ctx, "reset_init:"+emailStr, 3, 15*time.Minute)
	if err != nil {
		return err
	}
	if !allowed {
		return ErrRateLimitExceeded
	}

	email, err := domain.NewEmail(emailStr)
	if err != nil {
		return err
	}

	logger := pkg.WithContext(ctx).With(zap.String("email", emailStr))
	logger.Info("initiating password reset")

	user, _ := s.readRepo.FindByEmail(ctx, email)
	if user == nil {
		// We return nil to avoid email enumeration
		logger.Info("password reset initiated for non-existent user")
		return nil
	}

	plaintextToken := uuid.New().String()
	hashBytes := sha256.Sum256([]byte(plaintextToken))
	hashedToken := hex.EncodeToString(hashBytes[:])

	// Invalidate previous tokens
	if err := s.writeRepo.DeleteResetTokensByUserID(ctx, user.ID); err != nil {
		return err
	}

	token := domain.ResetToken{
		Token:     hashedToken,
		UserID:    user.ID,
		ExpiresAt: time.Now().Add(1 * time.Hour),
	}

	if err := s.writeRepo.SaveResetToken(ctx, &token); err != nil {
		return err
	}

	if s.publisher != nil {
		operation := func() error {
			return s.publisher.PublishPasswordReset(ctx, string(email), plaintextToken)
		}

		if err := backoff.Retry(operation, backoff.WithContext(backoff.NewExponentialBackOff(), ctx)); err != nil {
			return err
		}
		logger.Info("password reset event published successfully")
	}

	return nil
}

func (s *AuthCommandService) CompletePasswordReset(ctx context.Context, plaintextTokenStr, newPassword string) error {
	// The key here can be either IP address or hashed token. We will use plain text token to prevent brute forcing a specific token.
	allowed, err := s.limiter.Allow(ctx, "reset_complete:"+plaintextTokenStr, 5, 15*time.Minute)
	if err != nil {
		return err
	}
	if !allowed {
		return ErrRateLimitExceeded
	}

	if err := domain.ValidatePasswordComplexity(newPassword); err != nil {
		return err
	}

	hashBytes := sha256.Sum256([]byte(plaintextTokenStr))
	hashedTokenStr := hex.EncodeToString(hashBytes[:])

	token, err := s.writeRepo.FindResetToken(ctx, hashedTokenStr)
	if err != nil || token == nil || token.IsExpired() {
		return ErrInvalidToken
	}

	user, err := s.readRepo.FindByID(ctx, token.UserID)
	if err != nil || user == nil {
		return ErrInvalidToken
	}

	hash, err := domain.HashPassword(newPassword)
	if err != nil {
		return err
	}

	user.PasswordHash = hash
	user.UpdatedAt = time.Now()

	if err := s.writeRepo.Update(ctx, user); err != nil {
		return err
	}

	return s.writeRepo.DeleteResetToken(ctx, hashedTokenStr)
}
