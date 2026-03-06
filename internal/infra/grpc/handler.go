package grpc

import (
	"context"
	"errors"
	"time"

	"connectrpc.com/connect"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/trace"
	"go.uber.org/zap"

	"github.com/atrump/engineer-challenge/internal/app"
	pb "github.com/atrump/engineer-challenge/internal/infra/grpc/pkg"
	"github.com/atrump/engineer-challenge/internal/infra/grpc/pkg/authv1connect"
	"github.com/atrump/engineer-challenge/internal/pkg"
	"google.golang.org/protobuf/types/known/timestamppb"
)

var (
	ErrRateLimitExceeded = errors.New("rate limit exceeded, please try again later")
)

type AuthHandler struct {
	authv1connect.UnimplementedAuthServiceHandler
	commandService *app.AuthCommandService
	queryService   *app.AuthQueryService
	metrics        *AuthMetrics
}

func NewAuthHandler(commandService *app.AuthCommandService, queryService *app.AuthQueryService, metrics *AuthMetrics) *AuthHandler {
	return &AuthHandler{commandService: commandService, queryService: queryService, metrics: metrics}
}

func (h *AuthHandler) Register(ctx context.Context, req *connect.Request[pb.RegisterRequest]) (*connect.Response[pb.RegisterResponse], error) {
	start := time.Now()
	tr := otel.Tracer("auth-handler")
	ctx, span := tr.Start(ctx, "AuthHandler.Register")
	defer span.End()

	logger := pkg.WithContext(ctx)
	logger.Info("register request received",
		zap.String("email", req.Msg.Email),
	)

	id, err := h.commandService.Register(ctx, app.RegisterCommand{
		Email:    req.Msg.Email,
		Password: req.Msg.Password,
	})
	if err != nil {
		recordError(span, err)
		if h.metrics != nil {
			h.metrics.RecordRequest(ctx, "Register", "error", time.Since(start).Seconds())
		}
		if errors.Is(err, app.ErrUserAlreadyExists) {
			logger.Warn("user already exists", zap.String("email", req.Msg.Email), zap.Error(err))
			return nil, connect.NewError(connect.CodeAlreadyExists, err)
		}
		logger.Error("register failed", zap.String("email", req.Msg.Email), zap.Error(err))
		return nil, connect.NewError(connect.CodeInvalidArgument, err)
	}

	if h.metrics != nil {
		h.metrics.RecordRequest(ctx, "Register", "ok", time.Since(start).Seconds())
	}
	logger.Info("user registered successfully",
		zap.String("user_id", id.String()),
		zap.String("email", req.Msg.Email),
	)

	return connect.NewResponse(&pb.RegisterResponse{UserId: id.String()}), nil
}

func (h *AuthHandler) Login(ctx context.Context, req *connect.Request[pb.LoginRequest]) (*connect.Response[pb.LoginResponse], error) {
	start := time.Now()
	tr := otel.Tracer("auth-handler")
	ctx, span := tr.Start(ctx, "AuthHandler.Login")
	defer span.End()

	logger := pkg.WithContext(ctx)

	span.SetAttributes(
		attribute.String("auth.email", req.Msg.Email),
	)

	logger.Info("login request received",
		zap.String("email", req.Msg.Email),
	)

	at, rt, exp, err := h.queryService.Login(ctx, app.LoginQuery{
		Email:    req.Msg.Email,
		Password: req.Msg.Password,
	})
	if err != nil {
		recordError(span, err)
		if h.metrics != nil {
			h.metrics.RecordRequest(ctx, "Login", "error", time.Since(start).Seconds())
		}
		if errors.Is(err, app.ErrInvalidCredentials) {
			logger.Warn("invalid credentials", zap.String("email", req.Msg.Email), zap.Error(err))
			return nil, connect.NewError(connect.CodeUnauthenticated, err)
		}
		logger.Error("login failed", zap.String("email", req.Msg.Email), zap.Error(err))
		return nil, connect.NewError(connect.CodeInvalidArgument, err)
	}

	if h.metrics != nil {
		h.metrics.RecordRequest(ctx, "Login", "ok", time.Since(start).Seconds())
	}
	logger.Info("login succeeded",
		zap.String("email", req.Msg.Email),
		zap.Time("access_expires_at", exp),
	)

	return connect.NewResponse(&pb.LoginResponse{
		AccessToken:  at,
		RefreshToken: rt,
		ExpiresAt:    timestamppb.New(exp),
	}), nil
}

func (h *AuthHandler) InitiatePasswordReset(ctx context.Context, req *connect.Request[pb.InitiatePasswordResetRequest]) (*connect.Response[pb.InitiatePasswordResetResponse], error) {
	start := time.Now()
	tr := otel.Tracer("auth-handler")
	ctx, span := tr.Start(ctx, "AuthHandler.InitiatePasswordReset")
	defer span.End()

	logger := pkg.WithContext(ctx)

	span.SetAttributes(
		attribute.String("auth.email", req.Msg.Email),
	)

	logger.Info("initiate password reset request received",
		zap.String("email", req.Msg.Email),
	)

	err := h.commandService.InitiatePasswordReset(ctx, req.Msg.Email)
	if err != nil {
		recordError(span, err)
		if h.metrics != nil {
			h.metrics.RecordRequest(ctx, "InitiatePasswordReset", "error", time.Since(start).Seconds())
		}
		logger.Error("initiate password reset failed", zap.String("email", req.Msg.Email), zap.Error(err))
		return nil, connect.NewError(connect.CodeInvalidArgument, err)
	}

	if h.metrics != nil {
		h.metrics.RecordRequest(ctx, "InitiatePasswordReset", "ok", time.Since(start).Seconds())
	}
	return connect.NewResponse(&pb.InitiatePasswordResetResponse{
		Message: "If email exists, a reset link will be sent",
	}), nil
}

func (h *AuthHandler) CompletePasswordReset(ctx context.Context, req *connect.Request[pb.CompletePasswordResetRequest]) (*connect.Response[pb.CompletePasswordResetResponse], error) {
	start := time.Now()
	tr := otel.Tracer("auth-handler")
	ctx, span := tr.Start(ctx, "AuthHandler.CompletePasswordReset")
	defer span.End()

	logger := pkg.WithContext(ctx)

	span.SetAttributes(
		attribute.String("auth.email", req.Msg.Token),
	)

	logger.Info("complete password reset request received")

	err := h.commandService.CompletePasswordReset(ctx, req.Msg.Token, req.Msg.NewPassword)
	if err != nil {
		recordError(span, err)
		if h.metrics != nil {
			h.metrics.RecordRequest(ctx, "CompletePasswordReset", "error", time.Since(start).Seconds())
		}
		if errors.Is(err, app.ErrInvalidToken) {
			logger.Warn("invalid reset token", zap.Error(err))
			return nil, connect.NewError(connect.CodeUnauthenticated, err)
		}
		logger.Error("complete password reset failed", zap.Error(err))
		return nil, connect.NewError(connect.CodeInvalidArgument, err)
	}

	if h.metrics != nil {
		h.metrics.RecordRequest(ctx, "CompletePasswordReset", "ok", time.Since(start).Seconds())
	}
	logger.Info("password reset completed successfully")

	return connect.NewResponse(&pb.CompletePasswordResetResponse{Success: true}), nil
}

func recordError(span trace.Span, err error) {
	if span == nil || err == nil {
		return
	}

	span.RecordError(err)
	span.SetAttributes(
		attribute.Bool("error", true),
		attribute.String("error.message", err.Error()),
	)
}

