package redis

import (
	"context"
	"testing"
	"time"

	"github.com/alicebob/miniredis/v2"
	"github.com/redis/go-redis/v9"
	"github.com/stretchr/testify/assert"
)

func TestRateLimiter(t *testing.T) {
	s, err := miniredis.Run()
	if err != nil {
		t.Fatalf("failed to run miniredis: %v", err)
	}
	defer s.Close()

	client := redis.NewClient(&redis.Options{
		Addr: s.Addr(),
	})
	rl := NewRateLimiter(client)
	ctx := context.Background()
	key := "test-user"
	limit := 3
	window := time.Second

	// First 3 should allow
	for i := 0; i < limit; i++ {
		allow, err := rl.Allow(ctx, key, limit, window)
		assert.NoError(t, err)
		assert.True(t, allow)
	}

	// 4th should block
	allow, err := rl.Allow(ctx, key, limit, window)
	assert.NoError(t, err)
	assert.False(t, allow)

	// Wait for expiration
	s.FastForward(window + time.Millisecond)

	// Should allow again
	allow, err = rl.Allow(ctx, key, limit, window)
	assert.NoError(t, err)
	assert.True(t, allow)
}
