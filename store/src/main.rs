mod admin;
mod api_docs;
mod auth;
mod config;
mod general;
mod healthcheck;
mod metrics;
mod state;

pub mod order;
pub mod product;

use axum::{middleware, Router};
use reqwest::Url;
use sqlx::postgres::PgPoolOptions;
use starfoundry_lib_eve_gateway::load_signature;
use std::sync::Arc;
use tokio::select;
use tower_http::compression::CompressionLayer;
use tower_http::decompression::RequestDecompressionLayer;
use tower::ServiceBuilder;
use tracing_subscriber::EnvFilter;
use utoipa_axum::router::OpenApiRouter;
use utoipa_scalar::{Scalar, Servable};
use utoipa::OpenApi;

use crate::api_docs::ApiDoc;
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

    let postgres = PgPoolOptions::new()
        .connect(&config.database_url)
        .await?;

    let shop_config = Arc::new(config.shop_config);
    let discord_url = Arc::new(config.discord_url);

    let decoding_key = load_signature(Url::parse(&config.gateway_jwk_url).unwrap()).await.unwrap();
    let decoding_key = Arc::new(decoding_key);

    let state = AppState {
        postgres,
        shop_config,
        discord_url,

        decoding_key,
    };

    tracing::info!("Starting app server on {}", config.app_address.local_addr().unwrap());
    tracing::info!("Starting service server on {}", config.service_address.local_addr().unwrap());

    select! {
        r = axum::serve(config.app_address, app(state.clone())) => {
            if r.is_err() {
                tracing::error!("Error in app thread, error: {:?}", r);
            }
        },
        r = axum::serve(config.service_address, service(state.clone())) => {
            if r.is_err() {
                tracing::error!("Error in service thread, error: {:?}", r);
            }
        },
    }

    Ok(())
}

fn app(
    state: AppState,
) -> Router {
    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/admin", admin::routes())
        .nest("/auth", auth::routes())
        .nest("/general", general::routes())
        .nest("/products", product::routes(state.clone()))
        .nest("/orders", order::routes())
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
