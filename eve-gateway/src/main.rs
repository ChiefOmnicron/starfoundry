//! EVE-Gateway services.
//! 
//! The service sis designed to abstract everything away from the EVE-API.
//! 
//! Current supported features:
//! - Login with EVE SSO
//! - Wraps a small set of EVE-API calls (see the OpenAPI Documentation)
//! - Caching of routes

#![forbid(
    missing_docs,
    clippy::missing_docs_in_private_items,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
)]
mod api_docs;
mod auth;
mod character;
mod config;
mod eve_client;
mod healthcheck;
mod metrics;
mod state;

pub use self::state::*;
pub use self::eve_client::ENV_CLIENT_ID;
pub use self::eve_client::ENV_SECRET_KEY;
pub use self::eve_client::ENV_CALLBACK;

use axum::{middleware, Router};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tokio::select;
use tower_http::compression::CompressionLayer;
use tower_http::decompression::RequestDecompressionLayer;
use tower::ServiceBuilder;
use tracing_subscriber::EnvFilter;
use utoipa_axum::router::OpenApiRouter;
use utoipa_scalar::{Scalar, Servable};
use utoipa::OpenApi;

use crate::config::Config;
use crate::api_docs::ApiDoc;
use crate::metrics::{setup_metrics_recorder, path_metrics};

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

    let postgres = PgPoolOptions::new()
        .connect(&config.database_uri)
        .await?;
    sqlx::migrate!().run(&postgres).await?;

    let shared_state = AppState {
        postgres,
        auth_domains: Arc::new(config.domains),
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
    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/auth", auth::routes())
        .nest("/.well-known", auth::well_known_routes())
        .nest("/characters", character::routes())
        .layer(
            ServiceBuilder::new()
                .layer(middleware::from_fn(path_metrics))
                .layer(RequestDecompressionLayer::new())
                .layer(CompressionLayer::new())
        )
        .with_state(state.clone())
        .split_for_parts();

    let router = router.merge(Scalar::with_url("/", api));

    let router_v1 = Router::new().nest("/v1", router.clone());
    let router_latest = Router::new().nest("/latest", router.clone());

    router
        .merge(router_v1)
        .merge(router_latest)
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
