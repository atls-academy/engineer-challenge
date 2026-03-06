package main

import (
	"context"
	"fmt"
	"net/http"
	"os"
	"os/signal"
	"syscall"
	"time"

	"connectrpc.com/connect"
	"github.com/jackc/pgx/v5/pgxpool"
	"github.com/redis/go-redis/v9"
	"github.com/rs/cors"
	"go.opentelemetry.io/contrib/instrumentation/net/http/otelhttp"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/exporters/otlp/otlpmetric/otlpmetrichttp"
	"go.opentelemetry.io/otel/exporters/otlp/otlptrace/otlptracehttp"
	"go.opentelemetry.io/otel/metric"
	"go.opentelemetry.io/otel/metric/noop"
	sdkmetric "go.opentelemetry.io/otel/sdk/metric"
	"go.opentelemetry.io/otel/sdk/resource"
	sdktrace "go.opentelemetry.io/otel/sdk/trace"
	"go.uber.org/zap"
	"golang.org/x/net/http2"
	"golang.org/x/net/http2/h2c"

	"github.com/atrump/engineer-challenge/internal/app"
	"github.com/atrump/engineer-challenge/internal/infra"
	"github.com/atrump/engineer-challenge/internal/infra/db"
	igrpc "github.com/atrump/engineer-challenge/internal/infra/grpc"
	"github.com/atrump/engineer-challenge/internal/infra/grpc/pkg/authv1connect"
	"github.com/atrump/engineer-challenge/internal/infra/limiter"
	"github.com/atrump/engineer-challenge/internal/infra/memory"
	iredis "github.com/atrump/engineer-challenge/internal/infra/redis"
	"github.com/atrump/engineer-challenge/internal/pkg"
	"github.com/atrump/engineer-challenge/internal/pkg/rabbitmq"
)

func main() {
	cfg := infra.Load()

	if err := pkg.InitLogger(); err != nil {
		fmt.Fprintf(os.Stderr, "failed to initialize logger: %v\n", err)
	}
	logger := pkg.Logger()
	defer pkg.Sync()

	ctx := context.Background()

	shutdownTracer, err := setupTracing(ctx)
	if err != nil {
		logger.Error("failed to initialize tracing", zap.Error(err))
	} else {
		defer func() {
			// context with timeout to gracefully shutdown tracer provider
			ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
			defer cancel()
			if err := shutdownTracer(ctx); err != nil {
				logger.Error("failed to shutdown tracer provider", zap.Error(err))
			}
		}()
	}

	shutdownMetrics, err := setupMetrics(ctx)
	if err != nil {
		logger.Error("failed to initialize metrics", zap.Error(err))
	} else {
		defer func() {
			ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
			defer cancel()
			if err := shutdownMetrics(ctx); err != nil {
				logger.Error("failed to shutdown metrics provider", zap.Error(err))
			}
		}()
	}

	// 1. Setup DB
	pool, err := pgxpool.New(context.Background(), cfg.PostgresURL)
	if err != nil {
		logger.Fatal("failed to connect to postgres", zap.Error(err), zap.String("dsn", cfg.PostgresURL))
	}
	defer pool.Close()

	// 2. Setup Redis
	rdb := redis.NewClient(&redis.Options{
		Addr: cfg.RedisAddr,
	})
	defer rdb.Close()

	logger.Info("infrastructure initialized",
		zap.String("postgres_url", cfg.PostgresURL),
		zap.String("redis_addr", cfg.RedisAddr),
		zap.String("rabbitmq_url", cfg.RabbitMQURL),
		zap.String("grpc_port", cfg.GRPCPort),
	)

	// 3. Setup RabbitMQ Publisher
	var rmqPublisher *rabbitmq.Publisher
	for i := 0; i < 5; i++ {
		rmqPublisher, err = rabbitmq.NewPublisher(cfg.RabbitMQURL)
		if err == nil {
			break
		}
		logger.Warn("failed to initialize rabbitmq publisher, retrying...", zap.Int("attempt", i+1), zap.Error(err))
		time.Sleep(2 * time.Second)
	}

	if err != nil {
		logger.Error("failed to initialize rabbitmq publisher after retries, password resets will fail", zap.Error(err))
	} else {
		defer rmqPublisher.Close()
	}

	// 4. Setup Layers
	repo := db.NewPostgresUserRepository(pool)
	sessionRepo := iredis.NewRedisSessionRepository(rdb)
	jwtManager := pkg.NewJWTManager(cfg.JWTSecret, cfg.AccessExpiry, cfg.RefreshExpiry)
	
	redisLimiter := iredis.NewRateLimiter(rdb)
	memoryLimiter := memory.NewRateLimiter()
	rateLimiter := limiter.NewFallbackRateLimiter(redisLimiter, memoryLimiter, logger)

	commandService := app.NewAuthCommandService(repo, jwtManager, rmqPublisher, rateLimiter)
	queryService := app.NewAuthQueryService(repo, jwtManager, sessionRepo, cfg.RefreshExpiry, rateLimiter)

	// Build metrics instruments (falls back to noop if provider was not initialised)
	authMetrics, err := igrpc.NewAuthMetrics(otel.GetMeterProvider().Meter("auth-service"))
	if err != nil {
		logger.Warn("failed to create auth metrics, using noop", zap.Error(err))
		noopMetrics, _ := igrpc.NewAuthMetrics(noop.NewMeterProvider().Meter(""))
		authMetrics = noopMetrics
	}

	authHandler := igrpc.NewAuthHandler(commandService, queryService, authMetrics)

	// 4. Setup Connect with OpenTelemetry HTTP instrumentation
	rateInterceptor := igrpc.NewRateLimitInterceptor(rateLimiter, authMetrics)
	mux := http.NewServeMux()
	path, handler := authv1connect.NewAuthServiceHandler(
		authHandler,
		connect.WithInterceptors(rateInterceptor),
	)
	mux.Handle(path, otelhttp.NewHandler(handler, "auth_service"))

	// Add CORS support
	corsHandler := cors.New(cors.Options{
		AllowedOrigins: []string{"http://localhost:3000"}, // Vite dev server
		AllowedMethods: []string{"GET", "POST", "OPTIONS"},
		AllowedHeaders: []string{"Content-Type", "Connect-Protocol-Version"},
	}).Handler(mux)

	// 5. Graceful Shutdown
	srv := &http.Server{
		Addr:    fmt.Sprintf(":%s", cfg.GRPCPort),
		Handler: h2c.NewHandler(corsHandler, &http2.Server{}),
	}

	go func() {
		logger.Info("server starting",
			zap.String("port", cfg.GRPCPort),
			zap.String("protocol", "Connect/gRPC over HTTP"),
		)
		if err := srv.ListenAndServe(); err != nil && err != http.ErrServerClosed {
			logger.Fatal("failed to serve", zap.Error(err))
		}
	}()

	quit := make(chan os.Signal, 1)
	signal.Notify(quit, os.Interrupt, syscall.SIGTERM)
	<-quit

	logger.Info("shutdown signal received")
	ctx, cancel := context.WithTimeout(context.Background(), cfg.ShutdownTimeout)
	defer cancel()
	if err := srv.Shutdown(ctx); err != nil {
		logger.Fatal("failed to shutdown server", zap.Error(err))
	}

	logger.Info("server stopped gracefully")
}

// setupTracing configures OpenTelemetry tracing and returns a shutdown function.
func setupTracing(ctx context.Context) (func(context.Context) error, error) {
	endpoint := os.Getenv("OTEL_EXPORTER_OTLP_ENDPOINT")
	if endpoint == "" {
		endpoint = "http://otel-collector:4318"
	}

	exporter, err := otlptracehttp.New(ctx,
		otlptracehttp.WithEndpointURL(endpoint),
	)
	if err != nil {
		return nil, err
	}

	res, err := resource.New(ctx,
		resource.WithFromEnv(),
		resource.WithProcess(),
		resource.WithTelemetrySDK(),
		resource.WithAttributes(
			attribute.String("service.name", "auth-service"),
		),
	)
	if err != nil {
		return nil, err
	}

	tp := sdktrace.NewTracerProvider(
		sdktrace.WithBatcher(exporter),
		sdktrace.WithResource(res),
	)

	otel.SetTracerProvider(tp)

	return tp.Shutdown, nil
}

// setupMetrics configures OpenTelemetry metrics and returns a shutdown function.
func setupMetrics(ctx context.Context) (func(context.Context) error, error) {
	endpoint := os.Getenv("OTEL_EXPORTER_OTLP_ENDPOINT")
	if endpoint == "" {
		endpoint = "http://otel-collector:4318"
	}

	exporter, err := otlpmetrichttp.New(ctx,
		otlpmetrichttp.WithEndpointURL(endpoint),
	)
	if err != nil {
		return nil, err
	}

	res, err := resource.New(ctx,
		resource.WithFromEnv(),
		resource.WithProcess(),
		resource.WithTelemetrySDK(),
		resource.WithAttributes(
			attribute.String("service.name", "auth-service"),
		),
	)
	if err != nil {
		return nil, err
	}

	mp := sdkmetric.NewMeterProvider(
		sdkmetric.WithReader(sdkmetric.NewPeriodicReader(exporter,
			sdkmetric.WithInterval(10*time.Second),
		)),
		sdkmetric.WithResource(res),
	)

	otel.SetMeterProvider(mp)

	return mp.Shutdown, nil
}

// Ensure metric.MeterProvider is referenced (imported for noop fallback).
var _ metric.MeterProvider = (*noop.MeterProvider)(nil)

