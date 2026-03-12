package app

import (
	"context"
	"errors"
	"fmt"
	"time"

	"github.com/Aidajy111/engineer-challenge/internal/domain"
	"github.com/Aidajy111/engineer-challenge/pkg/hash"
)

type LoginResponse struct {
	AccessToken  string
	RefreshToken string
}

func (s *AuthService) Login(ctx context.Context, email, password string) (*LoginResponse, error) {
	user, err := s.repo.GetByEmail(ctx, email)
	if err != nil {
		if errors.Is(err, domain.ErrUserNotFound) {
			return nil, domain.ErrInvalidCredentials
		}
		return nil, fmt.Errorf("login failed: %w", err)
	}

	err = hash.CheckPassword(password, user.PasswordHash)
	if err != nil {
		return nil, domain.ErrInvalidCredentials
	}

	// TODO: Генерация JWT токенов
	// Пока вернем заглушки, чтобы проверить работу

	// 15 минут
	accessToken, err := s.tokenManager.GenerateToken(user.ID, time.Minute*15)
	if err != nil {
		return nil, err
	}

	//  30 дней
	refreshToken, err := s.tokenManager.GenerateToken(user.ID, time.Hour*24*30)
	if err != nil {
		return nil, err
	}

	return &LoginResponse{
		AccessToken:  accessToken,
		RefreshToken: refreshToken,
	}, nil
}
