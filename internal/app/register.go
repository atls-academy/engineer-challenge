package app

import (
	"context"
	"errors"
	"fmt"
	"time"

	"github.com/Aidajy111/engineer-challenge/internal/domain"
	"github.com/Aidajy111/engineer-challenge/pkg/hash"
	"github.com/google/uuid"
)

func (s *AuthService) Register(ctx context.Context, user *domain.User) (string, error) {
	if user.Email == "" || user.PasswordHash == "" {
		return "", domain.ErrInvalidCredentials
	}

	if len(user.PasswordHash) < 8 {
		return "", domain.ErrInvalidCredentials
	}

	if err := domain.ValidatePassword(user.PasswordHash); err != nil {
		return "", domain.ErrWeakPassword
	}

	userExists, err := s.repo.GetByEmail(ctx, user.Email)
	if err != nil {
		if !errors.Is(err, domain.ErrUserNotFound) {
			return "", fmt.Errorf("failed to check existing user: %w", err)
		}
	}
	if userExists != nil {
		return "", domain.ErrUserExists
	}

	passHash, err := hash.PasswordHash(user.PasswordHash)
	if err != nil {
		return "", fmt.Errorf("failed to hash password: %w", err)
	}

	user.ID = uuid.New()
	user.PasswordHash = passHash
	user.CreatedAt = time.Now()
	user.Status = domain.StatusStandard

	return s.repo.CreateUser(ctx, user)
}
