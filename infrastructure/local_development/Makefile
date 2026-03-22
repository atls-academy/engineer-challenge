.PHONY: up down infra-up infra-down kratos-up mailhog-up redis-up monitoring-up kratos-down mailhog-down redis-down monitoring-down cleanup

COMPOSE = docker compose

up: kratos-up

infra-up: kratos-up mailhog-up redis-up

infra-down: kratos-down mailhog-down redis-down

kratos-up:
	$(COMPOSE) -f docker-compose.kratos.yaml up -d
mailhog-up:
	$(COMPOSE) -f docker-compose.mailhog.yaml up -d
redis-up:
	$(COMPOSE) -f docker-compose.redis.yaml up -d
monitoring-up:
	$(COMPOSE) -f docker-compose.monitoring.yaml up -d
kratos-down:
	$(COMPOSE) -f docker-compose.kratos.yaml down -v
mailhog-down:
	$(COMPOSE) -f docker-compose.mailhog.yaml down -v
redis-down:
	$(COMPOSE) -f docker-compose.redis.yaml down -v
monitoring-down:
	$(COMPOSE) -f docker-compose.monitoring.yaml down -v
down: kratos-down
cleanup:
	$(COMPOSE) \
	-f docker-compose.kratos.yaml \
	-f docker-compose.monitoring.yaml \
	-f docker-compose.mailhog.yaml \
	-f docker-compose.redis.yaml \
	down -v
