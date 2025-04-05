################################################################################
# ALL
################################################################################
FROM        clux/muslrust:nightly as builder-all

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
            strip target/x86_64-unknown-linux-gnu/release/starfoundry_bin-meta_webserver

################################################################################
# APPRAISAL only
################################################################################
FROM        clux/muslrust:nightly as builder-appraisal

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
FROM        node as builder-webapp

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
# webapp-appraisal
################################################################################
FROM        node as builder-webapp-appraisal

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
FROM        ubuntu:24.04 as api

COPY        --from=builder-all /usr/src/starfoundry/target/x86_64-unknown-linux-gnu/release/starfoundry_bin-api /usr/local/bin/api

RUN         apt-get update && \
            apt-get install -y coinor-libcbc3.1 ca-certificates && \
            ln -s /usr/lib/x86_64-linux-gnu/libCbcSolver.so.3.10.11 /usr/lib/x86_64-linux-gnu/libCbcSolver.so.3
RUN         apt-get clean && \
            rm -rf /var/lib/apt/lists/*

EXPOSE      10101
CMD         ["/usr/local/bin/api"]

################################################################################
# Running api-appraisal container
################################################################################
FROM        ubuntu:24.04 as api-appraisal

COPY        --from=builder-appraisal /usr/src/starfoundry/target/x86_64-unknown-linux-gnu/release/starfoundry_bin-api /usr/local/bin/api

RUN         apt-get update && \
            apt-get install -y coinor-libcbc3.1 ca-certificates && \
            ln -s /usr/lib/x86_64-linux-gnu/libCbcSolver.so.3.10.11 /usr/lib/x86_64-linux-gnu/libCbcSolver.so.3
RUN         apt-get clean && \
            rm -rf /var/lib/apt/lists/*

EXPOSE      10101
CMD         ["/usr/local/bin/api"]

################################################################################
# Running collector container
################################################################################
FROM        ubuntu:24.04 as collector

RUN         apt-get update && \
            apt-get install -y ca-certificates
RUN         apt-get clean && \
            rm -rf /var/lib/apt/lists/*

COPY        --from=builder-all /usr/src/starfoundry/target/x86_64-unknown-linux-gnu/release/starfoundry_bin-collector /usr/local/bin/collector

CMD         ["/usr/local/bin/collector"]

################################################################################
# Running event-worker container
################################################################################
FROM        ubuntu:24.04 as event-worker

RUN         apt-get update && \
            apt-get install -y ca-certificates unzip

RUN         apt-get clean && \
            rm -rf /var/lib/apt/lists/*

COPY        --from=builder-all /usr/src/starfoundry/target/x86_64-unknown-linux-gnu/release/starfoundry_bin-event_worker /usr/local/bin/event_worker

CMD         ["/usr/local/bin/event_worker"]

################################################################################
# Running event-worker appraisal container
################################################################################
FROM        ubuntu:24.04 as event-worker-appraisal

RUN         apt-get update && \
            apt-get install -y ca-certificates unzip

RUN         apt-get clean && \
            rm -rf /var/lib/apt/lists/*

COPY        --from=builder-appraisal /usr/src/starfoundry/target/x86_64-unknown-linux-gnu/release/starfoundry_bin-event_worker /usr/local/bin/event_worker

CMD         ["/usr/local/bin/event_worker"]

################################################################################
# Running meta-webserver container
################################################################################
FROM        ubuntu:24.04 as meta-webserver

RUN         apt-get update && \
            apt-get install -y ca-certificates
RUN         apt-get clean && \
            rm -rf /var/lib/apt/lists/*

COPY        --from=builder-all /usr/src/starfoundry/target/x86_64-unknown-linux-gnu/release/starfoundry_bin-meta_webserver /usr/local/bin/meta-webserver

CMD         ["/usr/local/bin/meta-webserver"]

################################################################################
# Running webapp container
################################################################################
FROM        nginx:stable-alpine as webapp

COPY        --from=builder-webapp /app/dist /usr/share/nginx/html
COPY        webapp/nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE      80
CMD         ["nginx", "-g", "daemon off;"]

################################################################################
# Running webapp appraisal container
################################################################################
FROM        nginx:stable-alpine as webapp-appraisal

COPY        --from=builder-webapp-appraisal /app/dist /usr/share/nginx/html
COPY        webapp/nginx_appraisal.conf /etc/nginx/conf.d/default.conf

EXPOSE      80
CMD         ["nginx", "-g", "daemon off;"]
