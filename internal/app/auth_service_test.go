package app

import (
	"context"
	"crypto/sha256"
	"encoding/hex"
	"testing"
	"time"

	"github.com/atrump/engineer-challenge/internal/domain"
	"github.com/google/uuid"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/mock"
)

type MockRateLimiter struct {
	mock.Mock
}

func (m *MockRateLimiter) Allow(ctx context.Context, key string, limit int, window time.Duration) (bool, error) {
	args := m.Called(ctx, key, limit, window)
	return args.Bool(0), args.Error(1)
}

func TestAuthService_Register(t *testing.T) {
	repo := new(MockUserRepository)
	tm := new(MockTokenManager)
	limiter := new(MockRateLimiter)
	service := NewAuthCommandService(repo, tm, nil, limiter)

	ctx := context.Background()
	cmd := RegisterCommand{
		Email:    "test@example.com",
		Password: "SecurePassword123!",
	}

	limiter.On("Allow", ctx, "register:"+cmd.Email, 3, 10*time.Minute).Return(true, nil)
	repo.On("FindByEmail", ctx, domain.Email(cmd.Email)).Return(nil, nil)
	repo.On("Save", ctx, mock.MatchedBy(func(u *domain.User) bool {
		return u.Email == domain.Email(cmd.Email)
	})).Return(nil)

	id, err := service.Register(ctx, cmd)

	assert.NoError(t, err)
	assert.NotEqual(t, uuid.Nil, uuid.UUID(id))
	repo.AssertExpectations(t)
}

func TestAuthService_Login(t *testing.T) {
	repo := new(MockUserRepository)
	tm := new(MockTokenManager)
	sessionRepo := new(MockSessionRepository)
	limiter := new(MockRateLimiter)
	queryService := NewAuthQueryService(repo, tm, sessionRepo, time.Hour, limiter)

	ctx := context.Background()
	email := domain.Email("test@example.com")
	password := "SecurePassword123!"
	
	hash, _ := domain.HashPassword(password)
	user := domain.NewUser(email, hash)

	limiter.On("Allow", ctx, "login:"+string(email), 5, 5*time.Minute).Return(true, nil)
	repo.On("FindByEmail", ctx, email).Return(user, nil)
	
	exp := time.Now().Add(time.Hour)
	tm.On("GeneratePair", user.ID).Return("access", "refresh", exp, nil)

	sessionRepo.On("Save", ctx, mock.AnythingOfType("*domain.Session")).Return(nil)

	at, rt, expires, err := queryService.Login(ctx, LoginQuery{
		Email:    string(email),
		Password: password,
	})

	assert.NoError(t, err)
	assert.Equal(t, "access", at)
	assert.Equal(t, "refresh", rt)
	assert.Equal(t, exp, expires)
	repo.AssertExpectations(t)
}

func TestAuthService_PasswordReset(t *testing.T) {
	repo := new(MockUserRepository)
	tm := new(MockTokenManager)
	limiter := new(MockRateLimiter)
	service := NewAuthCommandService(repo, tm, nil, limiter)

	ctx := context.Background()
	email := domain.Email("test@example.com")
	user := domain.NewUser(email, "hash")

	// Test Initiate
	limiter.On("Allow", ctx, "reset_init:"+string(email), 3, 15*time.Minute).Return(true, nil)
	repo.On("FindByEmail", ctx, email).Return(user, nil)
	repo.On("DeleteResetTokensByUserID", ctx, user.ID).Return(nil)
	repo.On("SaveResetToken", ctx, mock.MatchedBy(func(rt *domain.ResetToken) bool {
		return rt.UserID == user.ID && len(rt.Token) == 64 // SHA-256 hex string
	})).Return(nil)

	err := service.InitiatePasswordReset(ctx, string(email))
	assert.NoError(t, err)

	// Test Complete
	tokenStr := "valid-token"
	hashBytes := sha256.Sum256([]byte(tokenStr))
	hashedToken := hex.EncodeToString(hashBytes[:])

	newPassword := "NewSecurePassword123!"
	resetToken := &domain.ResetToken{
		Token:     hashedToken,
		UserID:    user.ID,
		ExpiresAt: time.Now().Add(time.Hour),
	}

	limiter.On("Allow", ctx, "reset_complete:"+tokenStr, 5, 15*time.Minute).Return(true, nil)
	repo.On("FindResetToken", ctx, hashedToken).Return(resetToken, nil)
	repo.On("FindByID", ctx, user.ID).Return(user, nil)
	repo.On("Update", ctx, mock.MatchedBy(func(u *domain.User) bool {
		return u.ID == user.ID
	})).Return(nil)
	repo.On("DeleteResetToken", ctx, hashedToken).Return(nil)

	err = service.CompletePasswordReset(ctx, tokenStr, newPassword)
	assert.NoError(t, err)

	repo.AssertExpectations(t)
}

func TestAuthService_PasswordReset_WeakPassword(t *testing.T) {
	repo := new(MockUserRepository)
	tm := new(MockTokenManager)
	limiter := new(MockRateLimiter)
	service := NewAuthCommandService(repo, tm, nil, limiter)

	ctx := context.Background()
	tokenStr := "valid-token"
	newPassword := "weak"

	limiter.On("Allow", ctx, "reset_complete:"+tokenStr, 5, 15*time.Minute).Return(true, nil)

	err := service.CompletePasswordReset(ctx, tokenStr, newPassword)
	assert.Error(t, err)
	assert.Contains(t, err.Error(), "8 characters")
}
