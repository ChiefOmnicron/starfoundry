FROM rust AS chef
RUN         cargo install cargo-chef
RUN         apt update && apt install -y clang-19 build-essential
WORKDIR     /app

FROM chef AS planner
COPY        ./Cargo.toml Cargo.toml
COPY        ./.sqlx ./.sqlx
COPY        ./eve-gateway ./eve-gateway
COPY        ./eve-gateway_lib ./eve-gateway_lib
COPY        ./industry ./industry
COPY        ./libs ./libs
COPY        ./meta_webserver ./meta_webserver
COPY        ./store ./store
RUN         cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
ENV         SQLX_OFFLINE=true

COPY        --from=planner /app/recipe.json recipe.json
RUN         cargo chef cook --release --recipe-path recipe.json

COPY        ./Cargo.toml Cargo.toml
COPY        ./.sqlx ./.sqlx
COPY        ./eve-gateway ./eve-gateway
COPY        ./eve-gateway_lib ./eve-gateway_lib
COPY        ./industry ./industry
COPY        ./libs ./libs
COPY        ./meta_webserver ./meta_webserver
COPY        ./store ./store

###############################################################################
#           eve_gateway_api
###############################################################################
FROM builder AS eve-gateway-api-builder
RUN         cargo build --bin starfoundry_bin-eve_gateway --release

FROM debian:bookworm-slim AS eve-gateway-api
COPY        --from=eve-gateway-api-builder /app/target/release/starfoundry_bin-eve_gateway /usr/local/bin/
CMD         ["/usr/local/bin/app"]

###############################################################################
#           store_api
###############################################################################
FROM builder AS store-api-builder
RUN         cargo build --bin starfoundry_bin-store --release

FROM debian:bookworm-slim AS store-api
COPY        --from=store-api-builder /app/target/release/starfoundry_bin-store /usr/local/bin/
CMD         ["/usr/local/bin/app"]

###############################################################################
#           store_webapp
###############################################################################
FROM node AS store-webapp-builder
WORKDIR     /app
COPY        webapp_store/package*.json ./
COPY        webapp_store/tsconfig*.json ./
COPY        webapp_store/vite.config.ts ./
COPY        webapp_store/index.html ./
COPY        webapp_store/src ./src
COPY        webapp_store/cypress ./cypress
COPY        webapp_store/public ./public
RUN         npm install -g npm@latest
RUN         npm install
RUN         npm run build

FROM        nginx:stable-alpine AS store-webapp
COPY        --from=store-webapp-builder /app/dist /usr/share/nginx/html
COPY        webapp_store/nginx.conf /etc/nginx/conf.d/default.conf
EXPOSE      80
CMD         ["nginx", "-g", "daemon off;"]
