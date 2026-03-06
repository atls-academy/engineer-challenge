package pkg

import (
	"testing"
	"time"

	"github.com/atrump/engineer-challenge/internal/domain"
	"github.com/golang-jwt/jwt/v5"
	"github.com/google/uuid"
	"github.com/stretchr/testify/assert"
)

func TestJWTManager_GeneratePair(t *testing.T) {
	secret := "secret"
	accessExpiry := time.Minute
	refreshExpiry := time.Hour
	manager := NewJWTManager(secret, accessExpiry, refreshExpiry)

	userID := domain.UserID(uuid.New())
	at, rt, expires, err := manager.GeneratePair(userID)

	assert.NoError(t, err)
	assert.NotEmpty(t, at)
	assert.NotEmpty(t, rt)
	assert.WithinDuration(t, time.Now().Add(accessExpiry), expires, time.Second)

	// Verify Access Token
	token, err := jwt.Parse(at, func(token *jwt.Token) (interface{}, error) {
		return []byte(secret), nil
	})
	assert.NoError(t, err)
	claims, ok := token.Claims.(jwt.MapClaims)
	assert.True(t, ok)
	assert.Equal(t, userID.String(), claims["sub"])

	// Verify Refresh Token
	token, err = jwt.Parse(rt, func(token *jwt.Token) (interface{}, error) {
		return []byte(secret), nil
	})
	assert.NoError(t, err)
	claims, ok = token.Claims.(jwt.MapClaims)
	assert.True(t, ok)
	assert.Equal(t, userID.String(), claims["sub"])
}
