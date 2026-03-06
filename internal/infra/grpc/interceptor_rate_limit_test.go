package grpc_test

import (
	"context"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"

	"connectrpc.com/connect"
	igrpc "github.com/atrump/engineer-challenge/internal/infra/grpc"
	pb "github.com/atrump/engineer-challenge/internal/infra/grpc/pkg"
	"github.com/atrump/engineer-challenge/internal/infra/grpc/pkg/authv1connect"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/mock"
)

// MockRateLimiter
type MockRateLimiter struct {
	mock.Mock
}

func (m *MockRateLimiter) Allow(ctx context.Context, key string, limit int, window time.Duration) (bool, error) {
	args := m.Called(ctx, key, limit, window)
	return args.Bool(0), args.Error(1)
}

// DummyAuthHandler implements authv1connect.AuthServiceHandler
type DummyAuthHandler struct {
	authv1connect.UnimplementedAuthServiceHandler
	CallCount int
}

func (h *DummyAuthHandler) Login(ctx context.Context, req *connect.Request[pb.LoginRequest]) (*connect.Response[pb.LoginResponse], error) {
	h.CallCount++
	return connect.NewResponse(&pb.LoginResponse{}), nil
}

func TestRateLimitInterceptor_Integration(t *testing.T) {
	limiter := new(MockRateLimiter)
	interceptor := igrpc.NewRateLimitInterceptor(limiter, nil)
	handler := &DummyAuthHandler{}

	path, muxHandler := authv1connect.NewAuthServiceHandler(
		handler,
		connect.WithInterceptors(interceptor),
	)

	mux := http.NewServeMux()
	mux.Handle(path, muxHandler)
	server := httptest.NewServer(mux)
	defer server.Close()

	client := authv1connect.NewAuthServiceClient(
		http.DefaultClient,
		server.URL,
	)

	ctx := context.Background()

	// 1. Normal Request (httptest defaults client IP to 127.0.0.1)
	limiter.On("Allow", mock.Anything, "global_ip_limit:127.0.0.1", 100, 1*time.Minute).Return(true, nil).Once()

	req := connect.NewRequest(&pb.LoginRequest{Email: "test@test.com", Password: "pwd"})
	_, err := client.Login(ctx, req)
	assert.NoError(t, err)
	assert.Equal(t, 1, handler.CallCount)

	// 2. Denied Request
	limiter.On("Allow", mock.Anything, "global_ip_limit:127.0.0.1", 100, 1*time.Minute).Return(false, nil).Once()

	req = connect.NewRequest(&pb.LoginRequest{Email: "test@test.com", Password: "pwd"})
	_, err = client.Login(ctx, req)
	assert.Error(t, err)
	assert.Contains(t, err.Error(), "rate limit exceeded")
	assert.Equal(t, 1, handler.CallCount) // Should not increment

	// 3. X-Forwarded-For overrides IP
	limiter.On("Allow", mock.Anything, "global_ip_limit:203.0.113.5", 100, 1*time.Minute).Return(true, nil).Once()

	req = connect.NewRequest(&pb.LoginRequest{Email: "test@test.com", Password: "pwd"})
	req.Header().Set("X-Forwarded-For", "203.0.113.5")
	_, err = client.Login(ctx, req)
	assert.NoError(t, err)
	assert.Equal(t, 2, handler.CallCount)

	limiter.AssertExpectations(t)
}
