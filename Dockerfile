# Build stage
FROM golang:alpine AS builder

RUN apk add --no-cache protoc protobuf-dev git

WORKDIR /app

COPY go.mod go.sum ./
RUN go mod download

COPY . .

# Generate gRPC code (assuming protoc is in the image)
RUN go install google.golang.org/protobuf/cmd/protoc-gen-go@latest && \
    go install google.golang.org/grpc/cmd/protoc-gen-go-grpc@latest && \
    mkdir -p internal/infra/grpc/pkg && \
    protoc --proto_path=api/proto/v1 \
    --go_out=internal/infra/grpc/pkg --go_opt=paths=source_relative \
    --go-grpc_out=internal/infra/grpc/pkg --go-grpc_opt=paths=source_relative \
    api/proto/v1/*.proto

RUN CGO_ENABLED=0 GOOS=linux go build -o /server cmd/server/main.go

# Final stage
FROM alpine:latest

RUN apk --no-cache add ca-certificates postgresql-client

COPY --from=builder /server /server
COPY --from=builder /app/migrations /migrations

EXPOSE 50051

CMD ["/server"]
