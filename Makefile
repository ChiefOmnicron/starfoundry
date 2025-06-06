GIT_REV     := ${shell git rev-parse --short HEAD}
DOCKER_REPO := 175554084336.dkr.ecr.eu-central-1.amazonaws.com

.PHONY: build
build:
	cargo clippy -- -D clippy::missing_docs_in_private_items \
					-D clippy::missing_safety_doc \
					-D clippy::missing_panics_doc \
					-D clippy::missing_errors_doc
	cargo test
	cargo build

.PHONY: docs
docs:
	cargo clippy
	cargo doc --no-deps --document-private-items --all-features

.PHONY: docs-open
docs-open:
	cargo clippy
	cargo doc --no-deps --document-private-items --all-features --open

.PHONY: debug
debug:
	cargo build

.PHONY: test
test:
	cargo test

.PHONY: release
release:
	cargo build --release

.PHONY: sqlx-prepare
sqlx-prepare:
	cargo sqlx prepare  --workspace -- --all-targets --all-features

run-web-dev:
	cd webapp; npm run serve

.PHONY: web-test
web-test: web-test-chrome web-test-firefox web-test-edge

.PHONY: web-test-chrome
web-test-chrome:
	docker run \
		-v ${PWD}/webapp:/webapp \
		-w /webapp \
		--user ${shell id -u} \
		--rm \
		--entrypoint cypress \
		cypress/included:14.3.2 \
		run --component --browser chrome

.PHONY: web-test-firefox
web-test-firefox:
	docker run \
		-v ${PWD}/webapp:/webapp \
		-w /webapp \
		--user ${shell id -u} \
		--rm \
		--entrypoint cypress \
		cypress/included:14.3.2 \
		run --component --browser firefox

.PHONY: web-test-edge
web-test-edge:
	docker run \
		-v ${PWD}/webapp:/webapp \
		-w /webapp \
		--user ${shell id -u} \
		--rm \
		--entrypoint cypress \
		cypress/included:14.3.2 \
		run --component --browser edge

.PHONY: web-test-dev
web-test-dev:
	docker run \
		-v ${PWD}/webapp:/webapp \
		-w /webapp \
		--user ${shell id -u} \
		--rm \
		--entrypoint cypress \
		cypress/included:14.3.2 \
		run --component --browser chrome --spec src/notification/overview/integration/**.cy.ts

aaa:
	echo ${VITE_SENTRY}
	docker build \
		-t ${DOCKER_REPO}/starfoundry/web-appraisal \
		--build-arg VITE_SENTRY=${VITE_SENTRY} \
		--target webapp-appraisal \
		.

.PHONY: docker-build
docker-build:
	docker build \
		-t ${DOCKER_REPO}/starfoundry/api \
		--target api \
		.
	docker build \
		-t ${DOCKER_REPO}/starfoundry/api-appraisal \
		--target api-appraisal \
		.
	docker build \
		-t ${DOCKER_REPO}/starfoundry/collector \
		--target collector \
		.
	docker build \
		-t ${DOCKER_REPO}/starfoundry/database-upgrade \
		--target database-upgrade \
		.
	docker build \
		-t ${DOCKER_REPO}/starfoundry/event-worker \
		--target event-worker \
		.
	docker build \
		-t ${DOCKER_REPO}/starfoundry/event-worker-appraisal \
		--target event-worker-appraisal \
		.
	docker build \
		-t ${DOCKER_REPO}/starfoundry/meta-webserver \
		--target meta-webserver \
		.
	docker build \
		-t ${DOCKER_REPO}/starfoundry/web \
		--build-arg VITE_SENTRY=${VITE_SENTRY} \
		--build-arg SENTRY_AUTH_TOKEN=${SENTRY_AUTH_TOKEN} \
		--target webapp \
		.
	docker build \
		-t ${DOCKER_REPO}/starfoundry/web-appraisal \
		--build-arg VITE_SENTRY=${VITE_SENTRY} \
		--build-arg SENTRY_AUTH_TOKEN=${SENTRY_AUTH_TOKEN} \
		--target webapp-appraisal \
		.

.PHONY: docker-push
docker-push: docker-build
	docker tag ${DOCKER_REPO}/starfoundry/api:latest ${DOCKER_REPO}/starfoundry/api:${GIT_REV}
	docker push ${DOCKER_REPO}/starfoundry/api:latest
	docker push ${DOCKER_REPO}/starfoundry/api:${GIT_REV}

	docker tag ${DOCKER_REPO}/starfoundry/api-appraisal:latest ${DOCKER_REPO}/starfoundry/api-appraisal:${GIT_REV}
	docker push ${DOCKER_REPO}/starfoundry/api-appraisal:latest
	docker push ${DOCKER_REPO}/starfoundry/api-appraisal:${GIT_REV}

	docker tag ${DOCKER_REPO}/starfoundry/collector:latest ${DOCKER_REPO}/starfoundry/collector:${GIT_REV}
	docker push ${DOCKER_REPO}/starfoundry/collector:latest
	docker push ${DOCKER_REPO}/starfoundry/collector:${GIT_REV}

	docker tag ${DOCKER_REPO}/starfoundry/database-upgrade:latest ${DOCKER_REPO}/starfoundry/database-upgrade:${GIT_REV}
	docker push ${DOCKER_REPO}/starfoundry/database-upgrade:latest
	docker push ${DOCKER_REPO}/starfoundry/database-upgrade:${GIT_REV}

	docker tag ${DOCKER_REPO}/starfoundry/event-worker:latest ${DOCKER_REPO}/starfoundry/event-worker:${GIT_REV}
	docker push ${DOCKER_REPO}/starfoundry/event-worker:latest
	docker push ${DOCKER_REPO}/starfoundry/event-worker:${GIT_REV}

	docker tag ${DOCKER_REPO}/starfoundry/event-worker-appraisal:latest ${DOCKER_REPO}/starfoundry/event-worker-appraisal:${GIT_REV}
	docker push ${DOCKER_REPO}/starfoundry/event-worker-appraisal:latest
	docker push ${DOCKER_REPO}/starfoundry/event-worker-appraisal:${GIT_REV}

	docker tag ${DOCKER_REPO}/starfoundry/meta-webserver:latest ${DOCKER_REPO}/starfoundry/meta-webserver:${GIT_REV}
	docker push ${DOCKER_REPO}/starfoundry/meta-webserver:latest
	docker push ${DOCKER_REPO}/starfoundry/meta-webserver:${GIT_REV}

	docker tag ${DOCKER_REPO}/starfoundry/web:latest ${DOCKER_REPO}/starfoundry/web:${GIT_REV}
	docker push ${DOCKER_REPO}/starfoundry/web:latest
	docker push ${DOCKER_REPO}/starfoundry/web:${GIT_REV}

	docker tag ${DOCKER_REPO}/starfoundry/web-appraisal:latest ${DOCKER_REPO}/starfoundry/web-appraisal:${GIT_REV}
	docker push ${DOCKER_REPO}/starfoundry/web-appraisal:latest
	docker push ${DOCKER_REPO}/starfoundry/web-appraisal:${GIT_REV}

.PHONY: docker-login
docker-login:
	aws ecr get-login-password --region eu-central-1 --profile sf_infra | docker login --username AWS --password-stdin ${DOCKER_REPO}
