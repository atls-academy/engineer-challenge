package main

import (
	"log"
	"net"
	"os"
	"os/signal"
	"syscall"

	"google.golang.org/grpc"
	"google.golang.org/grpc/reflection"

	authgrpc "github.com/Aidajy111/engineer-challenge/internal/transport/grpc"
	identityv1 "github.com/Aidajy111/engineer-challenge/internal/transport/grpc/gen"
)

func main() {
	port := os.Getenv("PORT")
	if port == "" {
		port = "5555"
	}
	lis, err := net.Listen("tcp", ":"+port)
	if err != nil {
		log.Fatalf("failed to listen: %v", err)
	}

	stop := make(chan os.Signal, 1)
	signal.Notify(stop, os.Interrupt, syscall.SIGTERM)

	s := grpc.NewServer()
	identityv1.RegisterIdentityServiceServer(s, &authgrpc.Server{})
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
