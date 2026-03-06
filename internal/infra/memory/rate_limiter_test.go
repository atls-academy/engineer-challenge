package memory_test

import (
	"context"
	"testing"
	"time"

	"github.com/atrump/engineer-challenge/internal/infra/memory"
	"github.com/stretchr/testify/assert"
)

func TestRateLimiter_Allow(t *testing.T) {
	limiter := memory.NewRateLimiter()
	defer limiter.Close()
	ctx := context.Background()

	key := "test_ip_1"
	window := 50 * time.Millisecond
	limit := 3

	// 1st request - should be allowed
	allowed, err := limiter.Allow(ctx, key, limit, window)
	assert.NoError(t, err)
	assert.True(t, allowed)

	// 2nd request - should be allowed
	allowed, err = limiter.Allow(ctx, key, limit, window)
	assert.NoError(t, err)
	assert.True(t, allowed)

	// 3rd request - should be allowed
	allowed, err = limiter.Allow(ctx, key, limit, window)
	assert.NoError(t, err)
	assert.True(t, allowed)

	// 4th request - should be denied (limit exceeded)
	allowed, err = limiter.Allow(ctx, key, limit, window)
	assert.NoError(t, err)
	assert.False(t, allowed)

	// Wait for the window to expire
	time.Sleep(window + 10*time.Millisecond)

	// 5th request - should be allowed again (window reset)
	allowed, err = limiter.Allow(ctx, key, limit, window)
	assert.NoError(t, err)
	assert.True(t, allowed)
}

func TestRateLimiter_MultipleKeys(t *testing.T) {
	limiter := memory.NewRateLimiter()
	defer limiter.Close()
	ctx := context.Background()
	window := 1 * time.Second

	// Key A
	allowed, _ := limiter.Allow(ctx, "A", 1, window)
	assert.True(t, allowed)
	allowed, _ = limiter.Allow(ctx, "A", 1, window)
	assert.False(t, allowed) // Exceeded

	// Key B should remain unaffected
	allowed, _ = limiter.Allow(ctx, "B", 1, window)
	assert.True(t, allowed)
}
