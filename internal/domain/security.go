package domain

import (
	"context"
	"time"
)

type TokenManager interface {
	GeneratePair(userID UserID) (accessToken string, refreshToken string, expiresAt time.Time, err error)
}

type RateLimiter interface {
	Allow(ctx context.Context, key string, limit int, window time.Duration) (bool, error)
}
