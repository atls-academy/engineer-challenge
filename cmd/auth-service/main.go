package main

import (
	"log"
	"net"
	"os"
	"os/signal"
	"syscall"
	"time"

	_ "github.com/lib/pq"

	"github.com/Aidajy111/engineer-challenge/internal/app"
	"github.com/Aidajy111/engineer-challenge/internal/infra/db/postgres"
	"github.com/Aidajy111/engineer-challenge/pkg/jwt"
	"github.com/jmoiron/sqlx"
	"google.golang.org/grpc"
	"google.golang.org/grpc/reflection"

	authgrpc "github.com/Aidajy111/engineer-challenge/internal/transport/grpc"
	identityv1 "github.com/Aidajy111/engineer-challenge/internal/transport/grpc/gen"
)

func main() {
	// todo пока хардкодинг, потом можно на конфиги
	dsn := os.Getenv("DATABASE_URL")
	if dsn == "" {
		dsn = "postgres://postgres:postgres@localhost:5432/postgres?sslmode=disable"
	}
	secretKey := os.Getenv("JWT_SECRET")
	if secretKey == "" {
		secretKey = "super-puper-secret-key"
	}
	env := os.Getenv("ENV")

	port := os.Getenv("PORT")
	if port == "" {
		port = "5555"
	}

	// Подключение к БД
	db, err := sqlx.Connect("postgres", dsn)
	if err != nil {
		log.Fatalf("failed to connect to db: %v", err)
	}
	defer db.Close()

	// Миграция
	log.Println("Running database migrations...")
	if err := postgres.RunMigrations(dsn, env); err != nil {
		log.Fatal("Migrations error: ", err)
	}

	db.SetMaxOpenConns(25)
	db.SetMaxIdleConns(25)
	db.SetConnMaxLifetime(5 * time.Minute)

	// Слои
	repo := postgres.NewPostgres(db)
	tokenManeger := jwt.NewTokenManager(secretKey)
	authService := app.NewAuthService(repo, tokenManeger)

	// Настройка gRPC
	lis, err := net.Listen("tcp", ":"+port)
	if err != nil {
		log.Fatalf("failed to listen: %v", err)
	}

	// канал для gracefil shutdown
	stop := make(chan os.Signal, 1)
	signal.Notify(stop, os.Interrupt, syscall.SIGTERM)

	s := grpc.NewServer()
	serverHandler := authgrpc.NewServer(authService)
	identityv1.RegisterIdentityServiceServer(s, serverHandler)
	// reflection для отладки через Postman
	reflection.Register(s)

	go func() {
		log.Printf("gRPC Server listening on :%s\n", port)
		if err := s.Serve(lis); err != nil {
			log.Printf("failed to serve: %v", err)
		}
	}()

	<-stop

	log.Println("Shutting down gRPC server...")
	s.GracefulStop()
	log.Println("Server stopped")
}
