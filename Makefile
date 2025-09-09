GIT_REV     := ${shell git rev-parse --short HEAD}
DOCKER_REPO := ghcr.io/chiefomnicron/starfoundry

.PHONY: sqlx-prepare
sqlx-prepare:
	cargo sqlx prepare --workspace -- --all-targets --all-features

.PHONY: sqlx-check
sqlx-check:
	cargo sqlx prepare --check --workspace -- --all-targets --all-features

.PHONY: docker-build
docker-build: sqlx-check
	docker build \
		-t ${DOCKER_REPO}/eve-gateway/api \
		--target eve-gateway-api \
		.
	docker build \
		-t ${DOCKER_REPO}/store/api \
		--target store-api \
		.
	docker build \
		-t ${DOCKER_REPO}/store/webapp \
		--target store-webapp \
		.

.PHONY: docker-push
docker-push: docker-build
	docker tag ${DOCKER_REPO}/eve-gateway/api:latest ${DOCKER_REPO}/eve-gateway/api:${GIT_REV}
	docker push ${DOCKER_REPO}/eve-gateway/api:latest
	docker push ${DOCKER_REPO}/eve-gateway/api:${GIT_REV}

	docker tag ${DOCKER_REPO}/store/api:latest ${DOCKER_REPO}/store/api:${GIT_REV}
	docker push ${DOCKER_REPO}/store/api:latest
	docker push ${DOCKER_REPO}/store/api:${GIT_REV}

	docker tag ${DOCKER_REPO}/store/webapp:latest ${DOCKER_REPO}/store/webapp:${GIT_REV}
	docker push ${DOCKER_REPO}/store/webapp:latest
	docker push ${DOCKER_REPO}/store/webapp:${GIT_REV}

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
