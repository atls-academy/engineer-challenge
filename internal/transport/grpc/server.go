package grpc

import (
	"context"

	identityv1 "github.com/Aidajy111/engineer-challenge/internal/transport/grpc/gen"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

type Server struct {
	identityv1.UnimplementedIdentityServiceServer
}

func (s *Server) Register(ctx context.Context, req *identityv1.RegisterRequest) (*identityv1.RegisterResponse, error) {
	if req.GetEmail() == "" || req.GetPassword() == "" {
		return nil, status.Error(codes.InvalidArgument, "email and password are required")
	}

	return &identityv1.RegisterResponse{
		UserId: "temporary-uuid-123", // Пока заглушка
	}, nil
}

// Login реализует вход
func (s *Server) Login(ctx context.Context, req *identityv1.LoginRequest) (*identityv1.LoginResponse, error) {
	return &identityv1.LoginResponse{
		AccessToken:  "dummy-access-token",
		RefreshToken: "dummy-refresh-token",
	}, nil
}

func (s *Server) ForgotPassword(ctx context.Context, req *identityv1.ForgotPasswordRequest) (*identityv1.ForgotPasswordResponse, error) {
	return &identityv1.ForgotPasswordResponse{Success: false}, nil
}

func (s *Server) ResetPassword(ctx context.Context, req *identityv1.ResetPasswordRequest) (*identityv1.ResetPasswordResponse, error) {
	return &identityv1.ResetPasswordResponse{Success: true}, nil
}
