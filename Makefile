.PHONY: proto-gen docker-up docker-down migrate-up migrate-down build run test

PROTO_DIR=api/proto/v1
OUT_DIR=internal/infra/grpc/pkg

proto-gen:
	mkdir -p $(OUT_DIR)
	protoc --proto_path=$(PROTO_DIR) \
		--go_out=$(OUT_DIR) --go_opt=paths=source_relative \
		--go-grpc_out=$(OUT_DIR) --go-grpc_opt=paths=source_relative \
		--connect-go_out=$(OUT_DIR) --connect-go_opt=paths=source_relative \
		$(PROTO_DIR)/*.proto

docker-up:
	docker compose -f docker-compose.yaml -f docker-compose.observability.yaml up -d --build

docker-stop:
	docker compose -f docker-compose.yaml -f docker-compose.observability.yaml stop

docker-down:
	docker compose -f docker-compose.yaml -f docker-compose.observability.yaml down -v
build:
	go build -o bin/server cmd/server/main.go

run: build
	./bin/server

test:
	go test -v ./...

migrate-up:
	# Add migration command here (e.g., migrate -path migrations/ -database "postgres://auth_user:auth_password@localhost:5432/auth_db?sslmode=disable" up)
	@echo "Migration up"

migrate-down:
	@echo "Migration down"
