package grpc

import (
	"context"
	"net"
	"strings"
	"time"

	"connectrpc.com/connect"
	"github.com/atrump/engineer-challenge/internal/domain"
	"github.com/atrump/engineer-challenge/internal/pkg"
	"go.uber.org/zap"
)

// NewRateLimitInterceptor creates a gRPC interceptor that limits requests by client IP.
// Useful as a global soft-limit against spam/brute-force.
func NewRateLimitInterceptor(limiter domain.RateLimiter, metrics *AuthMetrics) connect.UnaryInterceptorFunc {
	return func(next connect.UnaryFunc) connect.UnaryFunc {
		return func(ctx context.Context, req connect.AnyRequest) (connect.AnyResponse, error) {
			logger := pkg.WithContext(ctx)
			
			ip := extractIP(req)
			if ip == "" {
				ip = "unknown"
			}

			// We allow 100 requests per 1 minute per IP address.
			// This is a soft limit that shouldn't affect normal users behind NAT,
			// but will quickly cut off automated scripts hitting the API.
			allowed, err := limiter.Allow(ctx, "global_ip_limit:"+ip, 100, 1*time.Minute)
			if err != nil {
				// If Redis is down, we probably want to allow the request to pass or 
				// log it heavily depending on strictness. Here we chose to fail open.
				logger.Warn("rate limiter failed, allowing request", zap.Error(err))
				return next(ctx, req)
			}

			if !allowed {
				logger.Warn("global ip rate limit exceeded", zap.String("ip", ip))
				if metrics != nil {
					metrics.RecordRateLimit(ctx, "GlobalRateLimit")
				}
				return nil, connect.NewError(connect.CodeResourceExhausted, ErrRateLimitExceeded)
			}

			return next(ctx, req)
		}
	}
}

// extractIP attempts to get the real client IP from the request headers
// or falls back to the peer address.
func extractIP(req connect.AnyRequest) string {
	// 1. Check X-Forwarded-For if behind a proxy
	if xff := req.Header().Get("X-Forwarded-For"); xff != "" {
		parts := strings.Split(xff, ",")
		if len(parts) > 0 {
			return strings.TrimSpace(parts[0])
		}
	}

	// 2. Fallback to Peer string
	peer := req.Peer().Addr
	host, _, err := net.SplitHostPort(peer)
	if err != nil {
		// If it's not host:port, return the raw peer (might be a unix socket etc)
		return peer
	}
	return host
}
