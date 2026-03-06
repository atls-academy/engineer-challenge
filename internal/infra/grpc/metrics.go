package grpc

import (
	"context"

	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/metric"
)

// AuthMetrics holds OTel instruments for the auth service.
type AuthMetrics struct {
	requestsTotal   metric.Int64Counter
	requestDuration metric.Float64Histogram
	rateLimitHits   metric.Int64Counter
}

// NewAuthMetrics initialises the metric instruments using the provided Meter.
// Pass metric.NewNoopMeterProvider().Meter("") in tests to avoid panics.
func NewAuthMetrics(meter metric.Meter) (*AuthMetrics, error) {
	requestsTotal, err := meter.Int64Counter(
		"auth_requests_total",
		metric.WithDescription("Total number of auth RPC requests"),
	)
	if err != nil {
		return nil, err
	}

	requestDuration, err := meter.Float64Histogram(
		"auth_request_duration_seconds",
		metric.WithDescription("Duration of auth RPC requests in seconds"),
		metric.WithExplicitBucketBoundaries(0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1, 2.5),
	)
	if err != nil {
		return nil, err
	}

	rateLimitHits, err := meter.Int64Counter(
		"auth_rate_limit_hits_total",
		metric.WithDescription("Total number of requests blocked by the rate limiter"),
	)
	if err != nil {
		return nil, err
	}

	return &AuthMetrics{
		requestsTotal:   requestsTotal,
		requestDuration: requestDuration,
		rateLimitHits:   rateLimitHits,
	}, nil
}

// RecordRequest records a completed RPC call with its method name, outcome
// ("ok" or "error"), and wall-clock duration in seconds.
func (m *AuthMetrics) RecordRequest(ctx context.Context, method, status string, durationSec float64) {
	attrs := metric.WithAttributes(
		attribute.String("method", method),
		attribute.String("status", status),
	)
	m.requestsTotal.Add(ctx, 1, attrs)
	m.requestDuration.Record(ctx, durationSec, metric.WithAttributes(
		attribute.String("method", method),
	))
}

// RecordRateLimit increments the rate-limit counter for the given method/scope.
func (m *AuthMetrics) RecordRateLimit(ctx context.Context, method string) {
	m.rateLimitHits.Add(ctx, 1, metric.WithAttributes(
		attribute.String("method", method),
	))
}
