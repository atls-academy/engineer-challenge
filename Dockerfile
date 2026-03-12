FROM golang:1.25.3-alpine AS builder

WORKDIR /app

COPY . .

RUN go build -o auth-service ./cmd/auth-service/main.go

FROM alpine:latest

WORKDIR /app

COPY --from=builder /app/auth-service .

COPY --from=builder /app/migrations ./migrations

COPY .env .

CMD ["./auth-service"]