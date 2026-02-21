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
COPY        ./worker_lib ./worker_lib
COPY        ./worker-eve_sde_parser ./worker-eve_sde_parser
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
COPY        ./worker_lib ./worker_lib
COPY        ./worker-eve_sde_parser ./worker-eve_sde_parser

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
RUN         apt-get install -y clang coinor-cbc coinor-libcbc-dev && \
            cargo build --bin starfoundry_bin-market --release

FROM ubuntu:26.04 AS market-api
WORKDIR     /usr/local/bin

RUN         apt-get update && \
            apt-get install -y ca-certificates curl coinor-libcbc3.1 && \
            cp /usr/lib/x86_64-linux-gnu/libCbcSolver.so.3.1 /usr/lib/x86_64-linux-gnu/libCbcSolver.so.3 && \
            apt-get clean

COPY        --from=market-api-builder /app/target/release/starfoundry_bin-market /usr/local/bin/app
CMD         ["/usr/local/bin/app"]

###############################################################################
#           market_worker
###############################################################################
FROM builder AS market-worker-builder
RUN         cargo build --bin starfoundry_bin-market_worker --release

FROM ubuntu:26.04 AS market-worker
WORKDIR     /usr/local/bin

RUN         apt-get update && \
            apt-get install -y ca-certificates curl && \
            apt-get clean

COPY        --from=market-worker-builder /app/target/release/starfoundry_bin-market_worker /usr/local/bin/app
CMD         ["/usr/local/bin/app"]

###############################################################################
#           industry_api
###############################################################################
FROM builder AS industry-api-builder
RUN         cargo build --bin starfoundry_bin-industry --release
RUN         cargo build --bin uuidv7_migration --release

FROM ubuntu:26.04 AS industry-api
WORKDIR     /usr/local/bin

RUN         apt-get update && \
            apt-get install -y ca-certificates curl && \
            apt-get clean

COPY        --from=industry-api-builder /app/target/release/starfoundry_bin-industry /usr/local/bin/app
COPY        --from=industry-api-builder /app/target/release/uuidv7_migration /usr/local/bin/migration
CMD         ["/usr/local/bin/app"]

###############################################################################
#           webapp_base
###############################################################################
FROM node AS webapp_base
ARG         SENTRY_AUTH_TOKEN
WORKDIR     /app

COPY        industry_webapp/vite.config.ts ./industry_webapp/vite.config.ts
COPY        industry_webapp/nginx.conf ./industry_webapp/nginx.conf
COPY        industry_webapp/package*.json ./industry_webapp/
COPY        industry_webapp/tsconfig*.json ./industry_webapp/
COPY        industry_webapp/index.html ./industry_webapp/index.html
COPY        industry_webapp/src ./industry_webapp/src
COPY        industry_webapp/public ./industry_webapp/public
COPY        industry_webapp/cypress ./industry_webapp/cypress

COPY        industry-hub_webapp/vite.config.ts ./industry-hub_webapp/vite.config.ts
COPY        industry-hub_webapp/nginx.conf ./industry-hub_webapp/nginx.conf
COPY        industry-hub_webapp/package*.json ./industry-hub_webapp/
COPY        industry-hub_webapp/tsconfig*.json ./industry-hub_webapp/
COPY        industry-hub_webapp/index.html ./industry-hub_webapp/index.html
COPY        industry-hub_webapp/src ./industry-hub_webapp/src
COPY        industry-hub_webapp/public ./industry-hub_webapp/public

COPY        webapp_components/package*.json ./webapp_components/
COPY        webapp_components/tsconfig*.json ./webapp_components/
COPY        webapp_components/src ./webapp_components/src

COPY        package*.json ./

RUN         npm install -g npm@latest
RUN         npm install

RUN         cd industry_webapp && \
                npm run build

RUN         cd industry-hub_webapp && \
                npm run build

###############################################################################
#           industry_webapp
###############################################################################
FROM        nginx:stable-alpine AS industry-webapp
COPY        --from=webapp_base /app/industry_webapp/dist /usr/share/nginx/html
COPY        --from=webapp_base /app/industry_webapp/nginx.conf /etc/nginx/conf.d/default.conf
EXPOSE      80
CMD         ["nginx", "-g", "daemon off;"]

###############################################################################
#           industry-hubs_webapp
###############################################################################
FROM        nginx:stable-alpine AS industry-hub-webapp
COPY        --from=webapp_base /app/industry-hub_webapp/dist /usr/share/nginx/html
COPY        --from=webapp_base /app/industry-hub_webapp/nginx.conf /etc/nginx/conf.d/default.conf
EXPOSE      80
CMD         ["nginx", "-g", "daemon off;"]
