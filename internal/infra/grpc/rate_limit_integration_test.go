package grpc_test

import (
	"context"
	"net"
	"testing"
	"time"

	"net/http"

	"connectrpc.com/connect"
	"github.com/alicebob/miniredis/v2"
	"github.com/atrump/engineer-challenge/internal/app"
	"github.com/atrump/engineer-challenge/internal/domain"
	igrpc "github.com/atrump/engineer-challenge/internal/infra/grpc"
	pb "github.com/atrump/engineer-challenge/internal/infra/grpc/pkg"
	"github.com/atrump/engineer-challenge/internal/infra/grpc/pkg/authv1connect"
	iredis "github.com/atrump/engineer-challenge/internal/infra/redis"
	"github.com/redis/go-redis/v9"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/mock"
	"golang.org/x/net/http2"
	"golang.org/x/net/http2/h2c"
)

// Mock representations for missing components
type mockRepo struct {
	mock.Mock
}

func (m *mockRepo) Save(ctx context.Context, user *domain.User) error { return nil }
func (m *mockRepo) FindByEmail(ctx context.Context, email domain.Email) (*domain.User, error) { 
	hash, err := domain.HashPassword("Password123!")
	if err != nil {
		return nil, err
	}
	return domain.NewUser(email, hash), nil
}
func (m *mockRepo) FindByID(ctx context.Context, id domain.UserID) (*domain.User, error) { return nil, nil }
func (m *mockRepo) Update(ctx context.Context, user *domain.User) error { return nil }
func (m *mockRepo) SaveResetToken(ctx context.Context, token *domain.ResetToken) error { return nil }
func (m *mockRepo) FindResetToken(ctx context.Context, tokenStr string) (*domain.ResetToken, error) { return nil, nil }
func (m *mockRepo) DeleteResetToken(ctx context.Context, tokenStr string) error { return nil }
func (m *mockRepo) DeleteResetTokensByUserID(ctx context.Context, userID domain.UserID) error { return nil }

type mockTM struct {
	mock.Mock
}

func (m *mockTM) GeneratePair(userID domain.UserID) (string, string, time.Time, error) {
	return "access", "refresh", time.Now().Add(time.Hour), nil
}

func TestRateLimiterIntegration(t *testing.T) {
	// Setup miniredis
	mr, err := miniredis.Run()
	if err != nil {
		t.Fatalf("an error '%s' was not expected when starting miniredis", err)
	}
	defer mr.Close()

	rdb := redis.NewClient(&redis.Options{
		Addr: mr.Addr(),
	})

	repo := new(mockRepo)
	tm := new(mockTM)
	sessionRepo := iredis.NewRedisSessionRepository(rdb)
	limiter := iredis.NewRateLimiter(rdb)
	
	commandService := app.NewAuthCommandService(repo, tm, nil, limiter)
	queryService := app.NewAuthQueryService(repo, tm, sessionRepo, time.Hour, limiter)
	authHandler := igrpc.NewAuthHandler(commandService, queryService, nil)

	mux := http.NewServeMux()
	path, handler := authv1connect.NewAuthServiceHandler(authHandler)
	mux.Handle(path, handler)

	srv := &http.Server{
		Handler: h2c.NewHandler(mux, &http2.Server{}),
	}
	
	listener, err := net.Listen("tcp", "127.0.0.1:0")
	assert.NoError(t, err)

	go func() {
		_ = srv.Serve(listener)
	}()
	defer srv.Close()

	client := authv1connect.NewAuthServiceClient(
		http.DefaultClient,
		"http://"+listener.Addr().String(),
	)

	ctx := context.Background()

	email := "test@example.com"
	password := "Password123!"

	// Attempt 5 successful logins
	for i := 0; i < 5; i++ {
		req := connect.NewRequest(&pb.LoginRequest{
			Email:    email,
			Password: password,
		})
		_, err := client.Login(ctx, req)
		assert.NoError(t, err, "Login %d should succeed", i+1)
	}

	// 6th attempt should fail due to rate limit
	req := connect.NewRequest(&pb.LoginRequest{
		Email:    email,
		Password: password,
	})
	_, err = client.Login(ctx, req)
	assert.Error(t, err, "6th login attempt should fail")
	assert.Contains(t, err.Error(), "rate limit exceeded")
}
