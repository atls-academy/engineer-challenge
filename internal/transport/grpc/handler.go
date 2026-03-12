package grpc

import (
	"context"

	"github.com/Aidajy111/engineer-challenge/internal/app"
	"github.com/Aidajy111/engineer-challenge/internal/domain"
	identityv1 "github.com/Aidajy111/engineer-challenge/internal/transport/grpc/gen"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

type Server struct {
	identityv1.UnimplementedIdentityServiceServer
	authService *app.AuthService
}

func NewServer(authService *app.AuthService) *Server {
	return &Server{
		authService: authService,
	}
}

func (s *Server) Register(ctx context.Context, req *identityv1.RegisterRequest) (*identityv1.RegisterResponse, error) {
	user := &domain.User{
		Email:        req.GetEmail(),
		PasswordHash: req.GetPassword(),
		DisplayName:  req.GetDisplayName(),
		Status:       domain.StatusStandard,
	}

	UserID, err := s.authService.Register(ctx, user)
	if err != nil {
		return nil, status.Error(codes.Internal, "failed to register user")
	}

	return &identityv1.RegisterResponse{
		UserId: UserID,
	}, nil
}

// Login реализует вход
func (s *Server) Login(ctx context.Context, req *identityv1.LoginRequest) (*identityv1.LoginResponse, error) {
	loginResponse, err := s.authService.Login(ctx, req.GetEmail(), req.GetPassword())
	if err != nil {
		return nil, status.Error(codes.Internal, "failed to login")
	}
	return &identityv1.LoginResponse{
		AccessToken:  loginResponse.AccessToken,
		RefreshToken: loginResponse.RefreshToken,
	}, nil
}

func (s *Server) ForgotPassword(ctx context.Context, req *identityv1.ForgotPasswordRequest) (*identityv1.ForgotPasswordResponse, error) {
	err := s.authService.ForgotPassword(ctx, req.GetEmail())
	if err != nil {
		return nil, status.Error(codes.Internal, "failed to forgot password")
	}
	return &identityv1.ForgotPasswordResponse{Success: true}, nil
}

func (s *Server) ResetPassword(ctx context.Context, req *identityv1.ResetPasswordRequest) (*identityv1.ResetPasswordResponse, error) {
	err := s.authService.ResetPassword(ctx, req.GetToken(), req.GetNewPassword())
	if err != nil {
		return nil, status.Error(codes.Internal, "failed to reset password")
	}
	return &identityv1.ResetPasswordResponse{Success: true}, nil
}
