.PHONY: local-dev
local-dev:
	docker compose up --build --watch

.PHONY: sqlx-prepare
sqlx-prepare:
	cargo sqlx prepare --workspace -- --all-targets --all-features

.PHONY: sqlx-check
sqlx-check:
	cargo sqlx prepare --check --workspace -- --all-targets --all-features

.PHONY: test
test: sqlx-prepare
	# local database on a NVMeSSD prevents `PoolTimeout` errors during execution
	SQLX_OFFLINE=true DATABASE_URL=postgresql://postgres:postgres@localhost:5555/postgres cargo test

.PHONY: test-database
test-database:
	docker run --rm -ti --name postgres_test -e POSTGRES_PASSWORD=postgres -v /tmp/postgres_testing:/var/lib/postgresql -p 5555:5432 -d postgres:18

.PHONY: web-industry-test-chrome
web-industry-test-chrome:
	docker run \
		-v ${PWD}/webapp_industry:/webapp \
		-w /webapp \
		--user ${shell id -u} \
		--rm \
		--entrypoint cypress \
		cypress/included:15.1.0 \
		run --component --browser chrome

.PHONY: web-store-test-chrome
web-store-test-chrome:
	docker run \
		-v ${PWD}/webapp_store:/webapp \
		-w /webapp \
		--user ${shell id -u} \
		--rm \
		--entrypoint cypress \
		cypress/included:15.1.0 \
		run --component --browser chrome
