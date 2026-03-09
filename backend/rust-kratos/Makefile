.PHONY: run dev up down infra-up infra-down mailhog-up mailhog-down cleanup

COMPOSE = docker compose
RUST_BIN = cargo
RUST_ARGS = run

INFRA_PATH = ../../infrastructure/local_development

run:
	$(RUST_BIN) $(RUST_ARGS)

dev: infra-up mailhog-up run

infra-up:
	$(MAKE) -C $(INFRA_PATH) infra-up

infra-down:
	$(MAKE) -C $(INFRA_PATH) infra-down

mailhog-up:
	$(MAKE) -C $(INFRA_PATH) mailhog-up

mailhog-down:
	$(MAKE) -C $(INFRA_PATH) mailhog-down

up:
	$(COMPOSE) -f docker-compose.app.yaml up -d --build

down:
	$(COMPOSE) -f docker-compose.app.yaml down -v

cleanup:
	$(MAKE) -C $(INFRA_PATH) cleanup
