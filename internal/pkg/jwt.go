package pkg

import (
	"time"

	"github.com/atrump/engineer-challenge/internal/domain"
	"github.com/golang-jwt/jwt/v5"
)

type TokenManager interface {
	domain.TokenManager
}

type jwtManager struct {
	secret         []byte
	accessExpiry   time.Duration
	refreshExpiry  time.Duration
}

func NewJWTManager(secret string, accessExpiry, refreshExpiry time.Duration) TokenManager {
	return &jwtManager{
		secret:        []byte(secret),
		accessExpiry:  accessExpiry,
		refreshExpiry: refreshExpiry,
	}
}

func (m *jwtManager) GeneratePair(userID domain.UserID) (string, string, time.Time, error) {
	now := time.Now()
	expiresAt := now.Add(m.accessExpiry)

	accessToken, err := jwt.NewWithClaims(jwt.SigningMethodHS256, jwt.MapClaims{
		"sub": userID.String(),
		"exp": expiresAt.Unix(),
		"iat": now.Unix(),
	}).SignedString(m.secret)
	if err != nil {
		return "", "", time.Time{}, err
	}

	refreshToken, err := jwt.NewWithClaims(jwt.SigningMethodHS256, jwt.MapClaims{
		"sub": userID.String(),
		"exp": now.Add(m.refreshExpiry).Unix(),
		"iat": now.Unix(),
	}).SignedString(m.secret)
	if err != nil {
		return "", "", time.Time{}, err
	}

	return accessToken, refreshToken, expiresAt, nil
}
