//! EVE-Gateway services.
//! 
//! The service sis designed to abstract everything away from the EVE-API.
//! 
//! Current supported features:
//! - Login with EVE SSO
//! - Wraps a small set of EVE-API calls (see the OpenAPI Documentation)
//! - Caching of routes
//! 

#![allow(clippy::redundant_field_names)]

pub mod api_docs;
pub mod auth;
pub mod config;
pub mod healthcheck;
pub mod metrics;
pub mod state;
pub mod utils;

pub mod asset;
pub mod character;
pub mod fitting;
pub mod industry;
pub mod item;
pub mod search;
pub mod structure;
pub mod system;

pub mod eve;
pub mod proxy;

use axum::{middleware, Router};
use prometheus_client::registry::Registry;
use sqlx::postgres::PgPoolOptions;
use starfoundry_lib_eve_client::EveApiClientMetric;
use std::sync::Arc;
use tokio::select;
use tower::ServiceBuilder;
use tracing_subscriber::EnvFilter;
use utoipa_axum::router::OpenApiRouter;
use utoipa_scalar::{Scalar, Servable};
use utoipa::OpenApi;

use crate::api_docs::ApiDoc;
use crate::config::Config;
use crate::item::services::load_items;
use crate::metrics::{Metric, path_metrics};
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

    if config.domains.is_empty() {
        tracing::error!("No domain configured. See README.md");
        return Err("No domain configured. See README.md".into());
    }

    let postgres = PgPoolOptions::new()
        .connect(&config.database_url)
        .await?;
    sqlx::migrate!().run(&postgres).await?;

    // load items in the background to not interrupt the startup phase
    let pool = postgres.clone();
    tokio::spawn(async move {
        tracing::info!("starting item cache population");
        let _ = load_items(&pool).await;
        tracing::info!("item cache populated");
    });

    let mut metric_registry = Registry::with_prefix("starfoundry_eve_gateway_api");
    let metric = Metric::default();
    metric.register(&mut metric_registry);

    let eve_client_metric = EveApiClientMetric::new();
    eve_client_metric.register(&mut metric_registry);

    let state = AppState {
        postgres,
        metric:         Arc::new(metric),
        eve_api_metric: Arc::new(eve_client_metric),
        auth_domains:   Arc::new(config.domains),
    };

    tracing::info!("Starting app server on {}", config.app_address.local_addr().unwrap());
    tracing::info!("Starting service server on {}", config.service_address.local_addr().unwrap());

    select! {
        r = axum::serve(config.app_address, app(state.clone())) => {
            if r.is_err() {
                tracing::error!("Error in app thread, error: {:?}", r);
            }
        },
        r = axum::serve(config.service_address, service(
            state.clone(),
            Arc::new(metric_registry),
        )) => {
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
        .nest("/.well-known", auth::well_known_routes())

        .nest("/auth", auth::routes())
        .nest("/characters", character::routes())
        .nest("/industry", industry::routes())
        .nest("/items", item::routes())
        .nest("/search", search::routes())
        .nest("/structures", structure::routes())
        .nest("/systems", system::routes())
        .nest("/eve", eve::routes())
        .nest("/proxy", proxy::routes())
        .layer(
            ServiceBuilder::new().layer(middleware::from_fn_with_state(state.clone(), path_metrics))
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
    state:    AppState,
    registry: Arc<Registry>,
) -> Router {
    Router::new()
        .nest("/health", healthcheck::routes().with_state(state))
        .route("/metrics", axum::routing::get(|| async move {
            metrics::route(registry)
        }))
}
