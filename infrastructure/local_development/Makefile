.PHONY: up down infra-up infra-down kratos-up hydra-up mailhog-up monitoring-up kratos-down hydra-down mailhog-down monitoring-down cleanup

COMPOSE = docker compose

up: kratos-up hydra-up

infra-up: kratos-up mailhog-up

infra-down: kratos-down mailhog-down

kratos-up:
	$(COMPOSE) -f docker-compose.kratos.yaml up -d

hydra-up:
	$(COMPOSE) -f docker-compose.hydra.yaml up -d

mailhog-up:
	$(COMPOSE) -f docker-compose.mailhog.yaml up -d

monitoring-up:
	$(COMPOSE) -f docker-compose.monitoring.yaml up -d

kratos-down:
	$(COMPOSE) -f docker-compose.kratos.yaml down -v

hydra-down:
	$(COMPOSE) -f docker-compose.hydra.yaml down -v

mailhog-down:
	$(COMPOSE) -f docker-compose.mailhog.yaml down -v

monitoring-down:
	$(COMPOSE) -f docker-compose.monitoring.yaml down -v

down: kratos-down hydra-down

cleanup:
	$(COMPOSE) \
	-f docker-compose.kratos.yaml \
	-f docker-compose.hydra.yaml \
	-f docker-compose.monitoring.yaml \
	-f docker-compose.mailhog.yaml \
	down -v
