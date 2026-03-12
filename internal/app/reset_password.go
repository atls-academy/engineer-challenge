package app

import (
	"context"
	"errors"
	"time"

	"github.com/Aidajy111/engineer-challenge/internal/domain"
	"golang.org/x/crypto/bcrypt"
)

func (s *AuthService) ResetPassword(ctx context.Context, token, newPassword string) error {
	user, err := s.repo.GetByResetToken(ctx, token)
	if err != nil {
		return domain.ErrInvalidCredentials // Токен не существует
	}

	if time.Now().After(user.ResetTokenExpires) {
		return errors.New("reset token expired")
	}

	newHash, _ := bcrypt.GenerateFromPassword([]byte(newPassword), 10)
	return s.repo.UpdatePassword(ctx, user.ID.String(), string(newHash))
}
