package limiter

import (
	"context"
	"time"

	"github.com/atrump/engineer-challenge/internal/domain"
	"go.uber.org/zap"
)

// FallbackRateLimiter wraps a primary and a secondary rate limiter.
// If the primary limiter fails with an error (e.g. database down),
// it falls back to the secondary limiter.
type FallbackRateLimiter struct {
	primary   domain.RateLimiter
	secondary domain.RateLimiter
	logger    *zap.Logger
}

// NewFallbackRateLimiter creates a new resilient fallback rate limiter.
func NewFallbackRateLimiter(primary, secondary domain.RateLimiter, logger *zap.Logger) *FallbackRateLimiter {
	return &FallbackRateLimiter{
		primary:   primary,
		secondary: secondary,
		logger:    logger,
	}
}

// Allow delegates to the primary limiter, and on error, delegates to the secondary fallback.
func (l *FallbackRateLimiter) Allow(ctx context.Context, key string, limit int, window time.Duration) (bool, error) {
	allowed, err := l.primary.Allow(ctx, key, limit, window)
	if err == nil {
		return allowed, nil
	}

	// Primary failed! Log and fallback
	l.logger.Warn("primary rate limiter failed, delegating to fallback", zap.Error(err), zap.String("key", key))
	
	allowedFallback, fallbackErr := l.secondary.Allow(ctx, key, limit, window)
	if fallbackErr != nil {
		l.logger.Error("secondary (fallback) rate limiter also failed", zap.Error(fallbackErr), zap.String("key", key))
		return false, fallbackErr
	}

	return allowedFallback, nil
}
