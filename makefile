COMPOSE_DEV = docker compose -f docker/docker-compose.yml
COMPOSE_TEST = $(COMPOSE_DEV) -f docker/docker-compose.test.yml

local:
	$(COMPOSE_DEV) up --build

migrate:
	$(COMPOSE_DEV) run --rm auth-manager diesel migration run

revert:
	$(COMPOSE_DEV) run --rm auth-manager diesel migration revert

t ?=
test:
	$(COMPOSE_TEST) run --rm test-runner bash -c "diesel database setup && cargo test $(t) -- --test-threads=5"
	$(COMPOSE_TEST) stop auth-db-test
