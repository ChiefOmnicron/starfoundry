################################################################################
# chef wrapper
################################################################################
FROM rust:1.93 AS chef
WORKDIR     /app

RUN         cargo install cargo-chef
RUN         apt update && apt install cmake clang -y

################################################################################
# chef planner
################################################################################
FROM chef AS planner
COPY        ./Cargo.toml Cargo.toml
COPY        ./.sqlx ./.sqlx
COPY        ./eve-gateway ./eve-gateway
COPY        ./eve-gateway_lib ./eve-gateway_lib
COPY        ./eve-gateway_worker ./eve-gateway_worker
COPY        ./gateway ./gateway
COPY        ./gateway_lib ./gateway_lib
COPY        ./gp_lib-types ./gp_lib-types
COPY        ./industry ./industry
COPY        ./industry_lib ./industry_lib
COPY        ./libs ./libs
COPY        ./market ./market
COPY        ./market_lib ./market_lib
COPY        ./market_worker ./market_worker
COPY        ./meta_webserver ./meta_webserver
COPY        ./store ./store
COPY        ./worker_lib ./worker_lib
COPY        ./worker-eve_sde_parser ./worker-eve_sde_parser
COPY        ./worker-store-cost ./worker-store-cost
# tmp
COPY        ./uuidv7_migration ./uuidv7_migration
RUN         cargo chef prepare --recipe-path recipe.json

################################################################################
# chef builder
################################################################################
FROM chef AS builder
ENV         SQLX_OFFLINE=true

COPY        --from=planner /app/recipe.json recipe.json
RUN         cargo chef cook --release --recipe-path recipe.json

COPY        ./Cargo.toml Cargo.toml
COPY        ./.sqlx ./.sqlx
COPY        ./eve-gateway ./eve-gateway
COPY        ./eve-gateway_lib ./eve-gateway_lib
COPY        ./eve-gateway_worker ./eve-gateway_worker
COPY        ./gateway ./gateway
COPY        ./gateway_lib ./gateway_lib
COPY        ./gp_lib-types ./gp_lib-types
COPY        ./industry ./industry
COPY        ./industry_lib ./industry_lib
COPY        ./libs ./libs
COPY        ./market ./market
COPY        ./market_lib ./market_lib
COPY        ./market_worker ./market_worker
COPY        ./meta_webserver ./meta_webserver
COPY        ./store ./store
COPY        ./worker_lib ./worker_lib
COPY        ./worker-eve_sde_parser ./worker-eve_sde_parser
COPY        ./worker-store-cost ./worker-store-cost

###############################################################################
#           eve_gateway_api
###############################################################################
FROM builder AS eve-gateway-api-builder
RUN         cargo build --bin starfoundry_bin-eve_gateway --release

FROM ubuntu:26.04 AS eve-gateway-api
WORKDIR     /usr/local/bin

RUN         apt-get update && \
            apt-get install -y ca-certificates curl && \
            apt-get clean

COPY        --from=eve-gateway-api-builder /app/target/release/starfoundry_bin-eve_gateway /usr/local/bin/app
CMD         ["/usr/local/bin/app"]

###############################################################################
#           eve_gateway_worker
###############################################################################
FROM builder AS eve-gateway-worker-builder
RUN         cargo build --bin starfoundry_bin-eve_gateway_worker --release

FROM ubuntu:26.04 AS eve-gateway-worker
WORKDIR     /usr/local/bin

RUN         apt-get update && \
            apt-get install -y ca-certificates curl && \
            apt-get clean

COPY        --from=eve-gateway-worker-builder /app/target/release/starfoundry_bin-eve_gateway_worker /usr/local/bin/app
CMD         ["/usr/local/bin/app"]

###############################################################################
#           gateway_api
###############################################################################
FROM builder AS gateway-api-builder
RUN         cargo build --bin starfoundry_bin-gateway --release

FROM ubuntu:26.04 AS gateway-api
WORKDIR     /usr/local/bin

RUN         apt-get update && \
            apt-get install -y ca-certificates curl && \
            apt-get clean

COPY        --from=gateway-api-builder /app/target/release/starfoundry_bin-gateway /usr/local/bin/app
CMD         ["/usr/local/bin/app"]

###############################################################################
#           market_api
###############################################################################
FROM builder AS market-api-builder
RUN         cargo build --bin starfoundry_bin-market --release

FROM ubuntu:26.04 AS market-api
WORKDIR     /usr/local/bin

RUN         apt-get update && \
            apt-get install -y ca-certificates curl && \
            apt-get clean

COPY        --from=market-api-builder /app/target/release/starfoundry_bin-market /usr/local/bin/app
CMD         ["/usr/local/bin/app"]

###############################################################################
#           store_api
###############################################################################
FROM builder AS store-api-builder
RUN         cargo build --bin starfoundry_bin-store --release

FROM ubuntu:26.04 AS store-api
WORKDIR     /usr/local/bin

RUN         apt-get update && \
            apt-get install -y ca-certificates curl && \
            apt-get clean

COPY        --from=store-api-builder /app/target/release/starfoundry_bin-store /usr/local/bin/app
CMD         ["/usr/local/bin/app"]

###############################################################################
#           store_worker_cost
###############################################################################
FROM builder AS store-worker-cost-builder
RUN         cargo build --bin starfoundry_bin-worker_store-cost --release

FROM ubuntu:26.04 AS store-worker-cost
WORKDIR     /usr/local/bin

RUN         apt-get update && \
            apt-get install -y ca-certificates curl && \
            apt-get clean

COPY        --from=store-worker-cost-builder /app/target/release/starfoundry_bin-worker_store-cost /usr/local/bin/app
CMD         ["/usr/local/bin/app"]

###############################################################################
#           store_webapp
###############################################################################
FROM node AS store-webapp-builder
ARG         VITE_SENTRY_STORE_DSN
ARG         SENTRY_AUTH_TOKEN
WORKDIR     /app
COPY        store_webapp/package*.json ./
COPY        store_webapp/tsconfig*.json ./
COPY        store_webapp/vite.config.ts ./
COPY        store_webapp/index.html ./
COPY        store_webapp/src ./src
COPY        store_webapp/cypress ./cypress
COPY        store_webapp/public ./public
RUN         npm install -g npm@latest
RUN         npm install
RUN         npm run build

FROM        nginx:stable-alpine AS store-webapp
COPY        --from=store-webapp-builder /app/dist /usr/share/nginx/html
COPY        store_webapp/nginx.conf /etc/nginx/conf.d/default.conf
EXPOSE      80
CMD         ["nginx", "-g", "daemon off;"]

###############################################################################
#           industry_api
###############################################################################
FROM builder AS industry-api-builder
RUN         cargo build --bin starfoundry_bin-industry --release

FROM ubuntu:26.04 AS industry-api
WORKDIR     /usr/local/bin

RUN         apt-get update && \
            apt-get install -y ca-certificates curl && \
            apt-get clean

COPY        --from=industry-api-builder /app/target/release/starfoundry_bin-industry /usr/local/bin/app
CMD         ["/usr/local/bin/app"]

###############################################################################
#           industry_webapp
###############################################################################
FROM node AS industry-webapp-builder
ARG         VITE_SENTRY_STORE_DSN
ARG         SENTRY_AUTH_TOKEN
WORKDIR     /app
COPY        industry_webapp/package*.json ./
COPY        industry_webapp/tsconfig*.json ./
COPY        industry_webapp/vite.config.ts ./
COPY        industry_webapp/index.html ./
COPY        industry_webapp/src ./src
COPY        industry_webapp/cypress ./cypress
COPY        industry_webapp/public ./public
RUN         npm install -g npm@latest
RUN         npm install
RUN         npm run build

FROM        nginx:stable-alpine AS industry-webapp
COPY        --from=industry-webapp-builder /app/dist /usr/share/nginx/html
COPY        industry_webapp/nginx.conf /etc/nginx/conf.d/default.conf
EXPOSE      80
CMD         ["nginx", "-g", "daemon off;"]
