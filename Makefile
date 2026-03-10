    .PHONY: generate
    generate:
        docker run --rm -v $(shell pwd):/app -w /app \
        thethingsindustries/protoc:latest \
        --proto_path=api/proto \
        --go_out=internal/transport/grpc/gen --go_opt=paths=source_relative \
        --go-grpc_out=internal/transport/grpc/gen --go-grpc_opt=paths=source_relative \
        api/proto/identity.proto