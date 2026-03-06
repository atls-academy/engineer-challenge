package grpc

import (
	"context"
	"testing"
	"time"

	"connectrpc.com/connect"
	"github.com/atrump/engineer-challenge/internal/app"
	"github.com/atrump/engineer-challenge/internal/domain"
	pb "github.com/atrump/engineer-challenge/internal/infra/grpc/pkg"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/mock"
)

type mockRepo struct {
	mock.Mock
}

func (m *mockRepo) Save(ctx context.Context, user *domain.User) error {
	return m.Called(ctx, user).Error(0)
}

func (m *mockRepo) FindByEmail(ctx context.Context, email domain.Email) (*domain.User, error) {
	args := m.Called(ctx, email)
	if args.Get(0) == nil {
		return nil, args.Error(1)
	}
	return args.Get(0).(*domain.User), args.Error(1)
}

func (m *mockRepo) FindByID(ctx context.Context, id domain.UserID) (*domain.User, error) {
	args := m.Called(ctx, id)
	if args.Get(0) == nil {
		return nil, args.Error(1)
	}
	return args.Get(0).(*domain.User), args.Error(1)
}

func (m *mockRepo) Update(ctx context.Context, user *domain.User) error {
	return m.Called(ctx, user).Error(0)
}

func (m *mockRepo) SaveResetToken(ctx context.Context, token *domain.ResetToken) error {
	return m.Called(ctx, token).Error(0)
}

func (m *mockRepo) FindResetToken(ctx context.Context, tokenStr string) (*domain.ResetToken, error) {
	args := m.Called(ctx, tokenStr)
	if args.Get(0) == nil {
		return nil, args.Error(1)
	}
	return args.Get(0).(*domain.ResetToken), args.Error(1)
}

func (m *mockRepo) DeleteResetToken(ctx context.Context, tokenStr string) error {
	return m.Called(ctx, tokenStr).Error(0)
}

func (m *mockRepo) DeleteResetTokensByUserID(ctx context.Context, userID domain.UserID) error {
	return m.Called(ctx, userID).Error(0)
}

type mockTM struct {
	mock.Mock
}

func (m *mockTM) GeneratePair(userID domain.UserID) (string, string, time.Time, error) {
	args := m.Called(userID)
	return args.String(0), args.String(1), args.Get(2).(time.Time), args.Error(3)
}

type mockSessionRepo struct {
	mock.Mock
}

func (m *mockSessionRepo) Save(ctx context.Context, session *domain.Session) error {
	return m.Called(ctx, session).Error(0)
}

func (m *mockSessionRepo) FindByToken(ctx context.Context, token string) (*domain.Session, error) {
	args := m.Called(ctx, token)
	if args.Get(0) == nil {
		return nil, args.Error(1)
	}
	return args.Get(0).(*domain.Session), args.Error(1)
}

func (m *mockSessionRepo) Delete(ctx context.Context, token string) error {
	return m.Called(ctx, token).Error(0)
}

type mockRateLimiter struct {
	mock.Mock
}

func (m *mockRateLimiter) Allow(ctx context.Context, key string, limit int, window time.Duration) (bool, error) {
	args := m.Called(ctx, key, limit, window)
	return args.Bool(0), args.Error(1)
}

func TestAuthHandler_Register(t *testing.T) {
	repo := new(mockRepo)
	tm := new(mockTM)
	sessionRepo := new(mockSessionRepo)
	limiter := new(mockRateLimiter)
	commandService := app.NewAuthCommandService(repo, tm, nil, limiter)
	queryService := app.NewAuthQueryService(repo, tm, sessionRepo, time.Hour, limiter)
	handler := NewAuthHandler(commandService, queryService, nil)

	ctx := context.Background()
	reqBody := &pb.RegisterRequest{
		Email:    "test@example.com",
		Password: "Password123!",
	}
	req := connect.NewRequest(reqBody)

	email, _ := domain.NewEmail(reqBody.Email)
	limiter.On("Allow", mock.Anything, "register:"+reqBody.Email, 3, 10*time.Minute).Return(true, nil)
	repo.On("FindByEmail", mock.Anything, email).Return(nil, nil)
	repo.On("Save", mock.Anything, mock.Anything).Return(nil)

	resp, err := handler.Register(ctx, req)

	assert.NoError(t, err)
	assert.NotEmpty(t, resp.Msg.UserId)
	repo.AssertExpectations(t)
}

func TestAuthHandler_Login(t *testing.T) {
	repo := new(mockRepo)
	tm := new(mockTM)
	sessionRepo := new(mockSessionRepo)
	limiter := new(mockRateLimiter)
	commandService := app.NewAuthCommandService(repo, tm, nil, limiter)
	queryService := app.NewAuthQueryService(repo, tm, sessionRepo, time.Hour, limiter)
	handler := NewAuthHandler(commandService, queryService, nil)

	ctx := context.Background()
	reqBody := &pb.LoginRequest{
		Email:    "test@example.com",
		Password: "Password123!",
	}
	req := connect.NewRequest(reqBody)

	email, _ := domain.NewEmail(reqBody.Email)
	hash, _ := domain.HashPassword(reqBody.Password)
	user := domain.NewUser(email, hash)

	limiter.On("Allow", mock.Anything, "login:"+reqBody.Email, 5, 5*time.Minute).Return(true, nil)
	repo.On("FindByEmail", mock.Anything, email).Return(user, nil)
	exp := time.Now().Add(time.Hour)
	tm.On("GeneratePair", user.ID).Return("access", "refresh", exp, nil)
	sessionRepo.On("Save", mock.Anything, mock.AnythingOfType("*domain.Session")).Return(nil)

	resp, err := handler.Login(ctx, req)

	assert.NoError(t, err)
	assert.Equal(t, "access", resp.Msg.AccessToken)
	assert.Equal(t, "refresh", resp.Msg.RefreshToken)
	repo.AssertExpectations(t)
}
