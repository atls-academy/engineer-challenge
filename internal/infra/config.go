package infra

import (
	"os"
	"time"
)

type Config struct {
	GRPCPort      string
	PostgresURL   string
	RedisAddr     string
	RabbitMQURL   string
	JWTSecret     string
	AccessExpiry     time.Duration
	RefreshExpiry    time.Duration
	ShutdownTimeout  time.Duration
}

func Load() *Config {
	return &Config{
		GRPCPort:      getEnv("GRPC_PORT", "50051"),
		PostgresURL:   getEnv("DATABASE_URL", "postgres://auth_user:auth_password@localhost:5432/auth_db?sslmode=disable"),
		RedisAddr:        getEnv("REDIS_ADDR", "localhost:6379"),
		RabbitMQURL:      getEnv("RABBITMQ_URL", "amqp://guest:guest@localhost:5672/"),
		JWTSecret:        getEnv("JWT_SECRET", "super-secret-key"),
		AccessExpiry:     time.Minute * 15,
		RefreshExpiry:    time.Hour * 24 * 7,
		ShutdownTimeout:  time.Second * 30,
	}
}

func getEnv(key, fallback string) string {
	if value, ok := os.LookupEnv(key); ok {
		return value
	}
	return fallback
}
