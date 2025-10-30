.PHONY: sqlx-prepare
sqlx-prepare:
	cargo sqlx prepare --workspace -- --all-targets --all-features

.PHONY: sqlx-check
sqlx-check:
	cargo sqlx prepare --check --workspace -- --all-targets --all-features

.PHONY: web-industry-test-chrome
web-industry-test-chrome:
	docker run \
		-v ${PWD}/webapp_industry:/webapp \
		-w /webapp \
		--user ${shell id -u} \
		--rm \
		--entrypoint cypress \
		cypress/included:15.1.0 \
		run --component --browser chrome --spec "src/routes/structures/_modal/-add.cy.tsx"

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
