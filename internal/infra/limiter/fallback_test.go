package limiter_test

import (
	"context"
	"errors"
	"testing"
	"time"

	"github.com/atrump/engineer-challenge/internal/infra/limiter"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/mock"
	"go.uber.org/zap/zaptest"
)

// MockRateLimiter
type MockRateLimiter struct {
	mock.Mock
}

func (m *MockRateLimiter) Allow(ctx context.Context, key string, limit int, window time.Duration) (bool, error) {
	args := m.Called(ctx, key, limit, window)
	return args.Bool(0), args.Error(1)
}

func TestFallbackRateLimiter_PrimarySuccess(t *testing.T) {
	primary := new(MockRateLimiter)
	secondary := new(MockRateLimiter)
	logger := zaptest.NewLogger(t)

	fallbackLimiter := limiter.NewFallbackRateLimiter(primary, secondary, logger)

	ctx := context.Background()
	key := "test_key"
	window := 1 * time.Minute
	limit := 10

	// Set up expectations: primary succeeds, secondary should never be called
	primary.On("Allow", ctx, key, limit, window).Return(true, nil).Once()

	allowed, err := fallbackLimiter.Allow(ctx, key, limit, window)

	assert.NoError(t, err)
	assert.True(t, allowed)

	primary.AssertExpectations(t)
	secondary.AssertNotCalled(t, "Allow")
}

func TestFallbackRateLimiter_PrimaryRateLimited(t *testing.T) {
	primary := new(MockRateLimiter)
	secondary := new(MockRateLimiter)
	logger := zaptest.NewLogger(t)

	fallbackLimiter := limiter.NewFallbackRateLimiter(primary, secondary, logger)

	ctx := context.Background()
	key := "test_key"
	window := 1 * time.Minute
	limit := 10

	// Set up expectations: primary succeeds but denies (rate limit exceeded)
	primary.On("Allow", ctx, key, limit, window).Return(false, nil).Once()

	allowed, err := fallbackLimiter.Allow(ctx, key, limit, window)

	assert.NoError(t, err)
	assert.False(t, allowed)

	primary.AssertExpectations(t)
	secondary.AssertNotCalled(t, "Allow")
}

func TestFallbackRateLimiter_PrimaryFails_SecondarySuccess(t *testing.T) {
	primary := new(MockRateLimiter)
	secondary := new(MockRateLimiter)
	logger := zaptest.NewLogger(t)

	fallbackLimiter := limiter.NewFallbackRateLimiter(primary, secondary, logger)

	ctx := context.Background()
	key := "test_key"
	window := 1 * time.Minute
	limit := 10

	primaryErr := errors.New("redis connection refused")

	// Set up expectations: primary fails, secondary takes over and succeeds
	primary.On("Allow", ctx, key, limit, window).Return(false, primaryErr).Once()
	secondary.On("Allow", ctx, key, limit, window).Return(true, nil).Once()

	allowed, err := fallbackLimiter.Allow(ctx, key, limit, window)

	assert.NoError(t, err)
	assert.True(t, allowed)

	primary.AssertExpectations(t)
	secondary.AssertExpectations(t)
}

func TestFallbackRateLimiter_PrimaryFails_SecondaryFails(t *testing.T) {
	primary := new(MockRateLimiter)
	secondary := new(MockRateLimiter)
	logger := zaptest.NewLogger(t)

	fallbackLimiter := limiter.NewFallbackRateLimiter(primary, secondary, logger)

	ctx := context.Background()
	key := "test_key"
	window := 1 * time.Minute
	limit := 10

	primaryErr := errors.New("redis connection refused")
	secondaryErr := errors.New("memory allocation failed")

	// Set up expectations: primary fails, secondary also fails
	primary.On("Allow", ctx, key, limit, window).Return(false, primaryErr).Once()
	secondary.On("Allow", ctx, key, limit, window).Return(false, secondaryErr).Once()

	allowed, err := fallbackLimiter.Allow(ctx, key, limit, window)

	assert.Error(t, err)
	assert.Equal(t, secondaryErr, err) // Should return the secondary error
	assert.False(t, allowed)

	primary.AssertExpectations(t)
	secondary.AssertExpectations(t)
}
