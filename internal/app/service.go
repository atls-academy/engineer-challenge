package app

import (
	"github.com/Aidajy111/engineer-challenge/internal/domain"
	"github.com/Aidajy111/engineer-challenge/pkg/jwt"
)

type AuthService struct {
	repo         domain.UserRepository
	tokenManager *jwt.TokenManager
}

func NewAuthService(r domain.UserRepository, manager *jwt.TokenManager) *AuthService {
	return &AuthService{repo: r, tokenManager: manager}
}

// Тут можно на случай расширения комманд добавить директории /commands и /queries и разделить на команды и запросы. Но так как в проекте всего 2 команды и 2 запроса решил этого не делать
