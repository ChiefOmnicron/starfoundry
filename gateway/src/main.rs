mod catch_all;
mod client;
mod config;
mod error;
mod healthcheck;
mod metrics;
mod state;

use axum::{middleware, Router};
use axum::routing::any;
use std::sync::Arc;
use tokio::select;
use tower_http::compression::CompressionLayer;
use tower_http::decompression::RequestDecompressionLayer;
use tower::ServiceBuilder;
use tracing_subscriber::EnvFilter;

use crate::state::AppState;
use crate::metrics::{path_metrics, setup_metrics_recorder};
use crate::catch_all::catch_all;
use crate::config::Config;

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
    let shared_state = AppState {
        routes: Arc::new(config.routes),
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
    // build our application with a route
    Router::new()
        .route("/{*key}", any(catch_all))
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
