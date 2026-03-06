package pkg

import (
	"context"
	"os"

	"go.opentelemetry.io/otel/trace"
	"go.uber.org/zap"
	"go.uber.org/zap/zapcore"
)

var (
	globalLogger *zap.Logger
)

// InitLogger initializes global structured logger with JSON output.
func InitLogger() error {
	cfg := zap.NewProductionConfig()
	cfg.Encoding = "json"
	cfg.OutputPaths = []string{"stdout"}
	cfg.ErrorOutputPaths = []string{"stderr"}
	cfg.EncoderConfig.TimeKey = "ts"
	cfg.EncoderConfig.LevelKey = "level"
	cfg.EncoderConfig.MessageKey = "msg"
	cfg.EncoderConfig.CallerKey = "caller"
	cfg.Level = zap.NewAtomicLevelAt(zapcore.InfoLevel)

	logger, err := cfg.Build()
	if err != nil {
		return err
	}

	globalLogger = logger
	return nil
}

// Logger returns global logger, falling back to std logger if not initialized.
func Logger() *zap.Logger {
	if globalLogger != nil {
		return globalLogger
	}

	logger, _ := zap.NewProduction()
	return logger
}

// WithContext enriches logger with trace/span ids from context.
func WithContext(ctx context.Context) *zap.Logger {
	logger := Logger()

	span := trace.SpanFromContext(ctx)
	sc := span.SpanContext()
	if !sc.IsValid() {
		return logger
	}

	return logger.With(
		zap.String("trace_id", sc.TraceID().String()),
		zap.String("span_id", sc.SpanID().String()),
	)
}

// Sync flushes buffered log entries.
func Sync() {
	if globalLogger != nil {
		_ = globalLogger.Sync() // ignore error on stdout/stderr
	}
}

func init() {
	// Best-effort initialization when imported without explicit InitLogger.
	_ = InitLogger()
	// ensure logger does not crash on missing env
	_ = os.Getenv("LOG_LEVEL")
}

