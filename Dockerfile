################################################################################
# ALL
################################################################################
FROM        rust:1.91.1 AS builder-all

ENV         SQLX_OFFLINE=true

WORKDIR     /usr/src/starfoundry

COPY        ./api/ ./api
COPY        ./collector/ ./collector
COPY        ./database_upgrade ./database_upgrade
COPY        ./event_worker/ ./event_worker
COPY        ./meta_webserver/ ./meta_webserver

COPY        ./libs/ ./libs

COPY        ./Cargo.toml Cargo.toml
COPY        ./.sqlx ./.sqlx

RUN         apt-get update && \
            apt-get install -y coinor-cbc coinor-libcbc-dev

RUN         cargo build --release --target x86_64-unknown-linux-gnu && \
            strip target/x86_64-unknown-linux-gnu/release/starfoundry_bin-api && \
            strip target/x86_64-unknown-linux-gnu/release/starfoundry_bin-collector && \
            strip target/x86_64-unknown-linux-gnu/release/starfoundry_bin-event_worker && \
            strip target/x86_64-unknown-linux-gnu/release/starfoundry_bin-meta_webserver && \
            strip target/x86_64-unknown-linux-gnu/release/starfoundry_bin-database_upgrade

################################################################################
# APPRAISAL only
################################################################################
FROM        rust:1.91.1 AS builder-appraisal

ENV         SQLX_OFFLINE=true

WORKDIR     /usr/src/starfoundry

COPY        ./api/ ./api
COPY        ./collector/ ./collector
COPY        ./database_upgrade ./database_upgrade
COPY        ./event_worker/ ./event_worker
COPY        ./meta_webserver/ ./meta_webserver

COPY        ./libs/ ./libs

COPY        ./Cargo.toml Cargo.toml
COPY        ./.sqlx ./.sqlx

RUN         apt-get update && \
            apt-get install -y coinor-cbc coinor-libcbc-dev

RUN         cd api; cargo build --release --target x86_64-unknown-linux-gnu --features "appraisal"; cd .. && \
            cd event_worker; cargo build --release --target x86_64-unknown-linux-gnu --features "appraisal"; cd .. && \
            cd meta_webserver; cargo build --release --target x86_64-unknown-linux-gnu; cd .. && \
            strip target/x86_64-unknown-linux-gnu/release/starfoundry_bin-api && \
            strip target/x86_64-unknown-linux-gnu/release/starfoundry_bin-event_worker && \
            strip target/x86_64-unknown-linux-gnu/release/starfoundry_bin-meta_webserver

################################################################################
# webapp
################################################################################
FROM        node:24 AS builder-webapp

ARG         VITE_SENTRY
ARG         SENTRY_AUTH_TOKEN

WORKDIR     /app

FROM chef AS planner
COPY        ./Cargo.toml Cargo.toml
COPY        ./.sqlx ./.sqlx
COPY        ./eve-gateway ./eve-gateway
COPY        ./eve-gateway_lib ./eve-gateway_lib
COPY        ./gateway ./gateway
COPY        ./gateway_lib ./gateway_lib
COPY        ./gp_lib-types ./gp_lib-types
COPY        ./industry ./industry
COPY        ./libs ./libs
COPY        ./meta_webserver ./meta_webserver
COPY        ./store ./store
COPY        ./worker-eve_sde_parser ./worker-eve_sde_parser
COPY        ./worker-store-cost ./worker-store-cost
RUN         cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
ENV         SQLX_OFFLINE=true

COPY        --from=planner /app/recipe.json recipe.json
RUN         cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json

COPY        ./Cargo.toml Cargo.toml
COPY        ./.sqlx ./.sqlx
COPY        ./eve-gateway ./eve-gateway
COPY        ./eve-gateway_lib ./eve-gateway_lib
COPY        ./gateway ./gateway
COPY        ./gateway_lib ./gateway_lib
COPY        ./gp_lib-types ./gp_lib-types
COPY        ./industry ./industry
COPY        ./libs ./libs
COPY        ./meta_webserver ./meta_webserver
COPY        ./store ./store
COPY        ./worker-eve_sde_parser ./worker-eve_sde_parser
COPY        ./worker-store-cost ./worker-store-cost

###############################################################################
#           eve_gateway_api
###############################################################################
FROM builder AS eve-gateway-api-builder
RUN         cargo build --bin starfoundry_bin-eve_gateway --target x86_64-unknown-linux-musl --release

FROM alpine:3.22 AS eve-gateway-api
WORKDIR     /usr/local/bin
COPY        --from=eve-gateway-api-builder /app/target/x86_64-unknown-linux-musl/release/starfoundry_bin-eve_gateway /usr/local/bin/app
CMD         ["/usr/local/bin/app"]

###############################################################################
#           gateway_api
###############################################################################
FROM builder AS gateway-api-builder
RUN         cargo build --bin starfoundry_bin-gateway --target x86_64-unknown-linux-musl --release

FROM alpine:3.22 AS gateway-api
WORKDIR     /usr/local/bin
RUN         apk update && apk add --no-cache curl
COPY        --from=gateway-api-builder /app/target/x86_64-unknown-linux-musl/release/starfoundry_bin-gateway /usr/local/bin/app
CMD         ["/usr/local/bin/app"]

###############################################################################
#           store_api
###############################################################################
FROM builder AS store-api-builder
RUN         cargo build --bin starfoundry_bin-store --target x86_64-unknown-linux-musl --release

FROM alpine:3.22 AS store-api
WORKDIR     /usr/local/bin
COPY        --from=store-api-builder /app/target/x86_64-unknown-linux-musl/release/starfoundry_bin-store /usr/local/bin/app
CMD         ["/usr/local/bin/app"]

###############################################################################
#           store_worker_cost
###############################################################################
FROM builder AS store-worker-cost-builder
RUN         cargo build --bin starfoundry_bin-worker_store-cost --target x86_64-unknown-linux-musl --release

FROM alpine:3.22 AS store-worker-cost
WORKDIR     /usr/local/bin
COPY        --from=store-worker-cost-builder /app/target/x86_64-unknown-linux-musl/release/starfoundry_bin-worker_store-cost /usr/local/bin/app
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

<<<<<<< HEAD
################################################################################
# webapp-appraisal
################################################################################
FROM        node:24 AS builder-webapp-appraisal

ARG         VITE_APPRAISAL=true
ARG         VITE_SENTRY
ARG         SENTRY_AUTH_TOKEN

WORKDIR     /app

COPY        webapp/package*.json ./
COPY        webapp/tsconfig*.json ./
COPY        webapp/vite.config.ts ./
COPY        webapp/index.html ./
COPY        webapp/src ./src
COPY        webapp/public ./public

RUN         npm install -g npm@latest
RUN         npm install
RUN         npm run build

################################################################################
# Running api container
################################################################################
FROM        ubuntu:24.04 AS api

COPY        --from=builder-all /usr/src/starfoundry/target/x86_64-unknown-linux-gnu/release/starfoundry_bin-api /usr/local/bin/api

RUN         apt-get update && \
            apt-get install -y coinor-libcbc3.1 ca-certificates curl && \
            ln -s /usr/lib/x86_64-linux-gnu/libCbcSolver.so.3.10.11 /usr/lib/x86_64-linux-gnu/libCbcSolver.so.3
RUN         apt-get clean && \
            rm -rf /var/lib/apt/lists/*

EXPOSE      10101
CMD         ["/usr/local/bin/api"]

################################################################################
# Running api-appraisal container
################################################################################
FROM        ubuntu:24.04 AS api-appraisal

COPY        --from=builder-appraisal /usr/src/starfoundry/target/x86_64-unknown-linux-gnu/release/starfoundry_bin-api /usr/local/bin/api

RUN         apt-get update && \
            apt-get install -y coinor-libcbc3.1 ca-certificates curl && \
            ln -s /usr/lib/x86_64-linux-gnu/libCbcSolver.so.3.10.11 /usr/lib/x86_64-linux-gnu/libCbcSolver.so.3
RUN         apt-get clean && \
            rm -rf /var/lib/apt/lists/*

EXPOSE      10101
CMD         ["/usr/local/bin/api"]

################################################################################
# Running collector container
################################################################################
FROM        ubuntu:24.04 AS collector

RUN         apt-get update && \
            apt-get install -y ca-certificates curl
RUN         apt-get clean && \
            rm -rf /var/lib/apt/lists/*

COPY        --from=builder-all /usr/src/starfoundry/target/x86_64-unknown-linux-gnu/release/starfoundry_bin-collector /usr/local/bin/collector

CMD         ["/usr/local/bin/collector"]


################################################################################
# Running database-upgrade container
################################################################################
FROM        ubuntu:24.04 AS database-upgrade

RUN         apt-get update && \
            apt-get install -y ca-certificates curl
RUN         apt-get clean && \
            rm -rf /var/lib/apt/lists/*

COPY        --from=builder-all /usr/src/starfoundry/target/x86_64-unknown-linux-gnu/release/starfoundry_bin-database_upgrade /usr/local/bin/database_upgrade

CMD         ["/usr/local/bin/database_upgrade"]

################################################################################
# Running event-worker container
################################################################################
FROM        ubuntu:24.04 AS event-worker

RUN         apt-get update && \
            apt-get install -y ca-certificates unzip curl

RUN         apt-get clean && \
            rm -rf /var/lib/apt/lists/*

COPY        --from=builder-all /usr/src/starfoundry/target/x86_64-unknown-linux-gnu/release/starfoundry_bin-event_worker /usr/local/bin/event_worker

CMD         ["/usr/local/bin/event_worker"]

################################################################################
# Running event-worker appraisal container
################################################################################
FROM        ubuntu:24.04 AS event-worker-appraisal

RUN         apt-get update && \
            apt-get install -y ca-certificates unzip curl

RUN         apt-get clean && \
            rm -rf /var/lib/apt/lists/*

COPY        --from=builder-appraisal /usr/src/starfoundry/target/x86_64-unknown-linux-gnu/release/starfoundry_bin-event_worker /usr/local/bin/event_worker

CMD         ["/usr/local/bin/event_worker"]

################################################################################
# Running meta-webserver container
################################################################################
FROM        ubuntu:24.04 AS meta-webserver

RUN         apt-get update && \
            apt-get install -y ca-certificates
RUN         apt-get clean && \
            rm -rf /var/lib/apt/lists/*

COPY        --from=builder-all /usr/src/starfoundry/target/x86_64-unknown-linux-gnu/release/starfoundry_bin-meta_webserver /usr/local/bin/meta-webserver

CMD         ["/usr/local/bin/meta-webserver"]

################################################################################
# Running webapp container
################################################################################
FROM        nginx:stable-alpine AS webapp

COPY        --from=builder-webapp /app/dist /usr/share/nginx/html
COPY        webapp/nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE      80
CMD         ["nginx", "-g", "daemon off;"]

################################################################################
# Running webapp appraisal container
################################################################################
FROM        nginx:stable-alpine AS webapp-appraisal

COPY        --from=builder-webapp-appraisal /app/dist /usr/share/nginx/html
COPY        webapp/nginx.conf /etc/nginx/conf.d/default.conf

=======
FROM        nginx:stable-alpine AS store-webapp
COPY        --from=store-webapp-builder /app/dist /usr/share/nginx/html
COPY        webapp_store/nginx.conf /etc/nginx/conf.d/default.conf
>>>>>>> ea95a81a (WIP)
EXPOSE      80
CMD         ["nginx", "-g", "daemon off;"]
