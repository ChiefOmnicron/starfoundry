// Avoid musl's default allocator due to lackluster performance
// https://nickb.dev/blog/default-musl-allocator-considered-harmful-to-performance
#[cfg(target_env = "musl")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

mod auth;
mod catch_all;
mod client;
mod config;
mod error;
mod healthcheck;
mod metrics;
mod state;

use axum::{middleware, Router};
use axum::routing::get;
use std::sync::Arc;
use tokio::select;
use tower_http::compression::CompressionLayer;
use tower_http::decompression::RequestDecompressionLayer;
use tower::ServiceBuilder;
use tracing_subscriber::EnvFilter;

use crate::auth::load_signature;
use crate::catch_all::*;
use crate::config::Config;
use crate::metrics::{path_metrics, setup_metrics_recorder};
use crate::state::AppState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    if cfg!(debug_assertions) {
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .init();
    } else {
        tracing_subscriber::fmt()
            .json()
            .with_env_filter(EnvFilter::from_default_env())
            .init();
    }

    let config = Config::load().await?;

    let decoding_key = load_signature(config.eve_gateway_jwk_url).await?;
    let decoding_key = Arc::new(decoding_key);

    let shared_state = AppState {
        routes: Arc::new(config.routes),

        decoding_key,
    };

    tracing::info!("Starting app server on {}", config.app_address.local_addr().unwrap());
    tracing::info!("Starting service server on {}", config.service_address.local_addr().unwrap());

    select! {
        r = axum::serve(config.app_address, app(shared_state.clone())) => {
            if r.is_err() {
                tracing::error!("Error in app thread, error: {:?}", r);
            }
        },
        r = axum::serve(config.service_address, service(shared_state.clone())) => {
            if r.is_err() {
                tracing::error!("Error in service thread, error: {:?}", r);
            }
        },
    }

    panic!("Error while execution, see logs")
}

fn app(
    state: AppState,
) -> Router {
    Router::new()
        .route("/.well-known/jwks", get(catch_all_well_known))
        .route("/auth/callback", get(catch_all_auth_callback))
        .route("/auth/login", get(catch_all_auth_login))
        .route("/auth/login/callback", get(catch_all_auth_login_callback))
        .route("/auth/token", get(catch_all_auth_token))
        .route("/{*key}",
            get(catch_all_generic_get)
                .delete(catch_all_generic_delete)
                .post(catch_all_generic_post)
                .put(catch_all_generic_put)
        )
        .layer(
            ServiceBuilder::new()
                .layer(middleware::from_fn(path_metrics))
                .layer(RequestDecompressionLayer::new())
                .layer(CompressionLayer::new())
        )
        .with_state(state.clone())
}

/// General service routes that do not need to be publicly accessible
fn service(
    state: AppState,
) -> Router {
    let metrics = setup_metrics_recorder();

    Router::new()
        .nest("/health", healthcheck::routes().with_state(state))
        .route("/metrics", axum::routing::get(|| async move {
            metrics::route(metrics)
        }))
}
