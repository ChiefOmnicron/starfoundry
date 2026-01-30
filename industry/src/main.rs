// Avoid musl's default allocator due to lackluster performance
// https://nickb.dev/blog/default-musl-allocator-considered-harmful-to-performance
#[cfg(target_env = "musl")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

mod api_docs;
mod config;
mod healthcheck;
mod industry;
mod industry_hub;
mod internal;
mod metrics;
mod project_group;
mod project;
mod state;
mod structure;

pub use self::state::*;

use axum::{middleware, Router};
use sqlx::postgres::PgPoolOptions;
use tokio::select;
use tower::ServiceBuilder;
use tracing_subscriber::EnvFilter;
use utoipa_axum::router::OpenApiRouter;
use utoipa_scalar::{Scalar, Servable};
use utoipa::OpenApi;

use crate::config::Config;
use crate::api_docs::ApiDoc;
use crate::metrics::{setup_metrics_recorder, path_metrics};

#[allow(dead_code)]
const SERVICE_NAME: &str = "SF_INDUSTRY";

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

    let pool = PgPoolOptions::new()
        .connect(&config.database_url)
        .await?;

    let state = AppState {
        pool,
    };

    rustls::crypto::aws_lc_rs::default_provider().install_default().unwrap();
    // configure certificate and private key used by https
    let tls_config = RustlsConfig::from_pem(
            config.mtls_cert.as_bytes().to_vec(),
            config.mtls_priv.as_bytes().to_vec(),
        )
        .await?;

    tracing::info!("Starting app server on {}", config.app_address.local_addr().unwrap());
    tracing::info!("Starting service server on {}", config.service_address.local_addr().unwrap());
    tracing::info!("Starting internal server on {}", config.internal_address.local_addr().unwrap());

    select! {
        r = axum_server::from_tcp_rustls(config.app_address, tls_config.clone()).serve(app(state.clone()).into_make_service()) => {
            if r.is_err() {
                tracing::error!("Error in app thread, error: {:?}", r);
            }
        },
        r = axum::serve(config.service_address, service(state.clone())) => {
            if r.is_err() {
                tracing::error!("Error in service thread, error: {:?}", r);
            }
        },
        r = axum_server::from_tcp_rustls(config.internal_address, tls_config).serve(internal(state.clone()).into_make_service()) => {
            if r.is_err() {
                tracing::error!("Error in internal thread, error: {:?}", r);
            }
        },
    }

    Ok(())
}

fn app(
    state: AppState,
) -> Router {
    // build our application with a route
    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/projects", project::routes(state.clone()))
        .nest("/project-groups", project_group::routes(state.clone()))
        .nest("/structures", structure::routes(state.clone()))
        .nest("/industry", industry::routes())
        .nest("/industry-hubs", industry_hub::routes(state.clone()))
        .layer(
            ServiceBuilder::new()
                .layer(middleware::from_fn(path_metrics))
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

fn internal(
    state: AppState,
) -> Router {
    let (router, _) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/internal", internal::routes())
        .with_state(state.clone())
        .split_for_parts();

    router
}

#[cfg(not(test))]
use starfoundry_lib_eve_gateway::EveGatewayClient;
#[cfg(not(test))]
pub fn eve_gateway_api_client() -> Result<EveGatewayClient, starfoundry_lib_eve_gateway::Error> {
    EveGatewayClient::new(SERVICE_NAME.into())
}
#[cfg(not(test))]
use starfoundry_lib_market::MarketClient;
#[cfg(not(test))]
pub fn market_api_client() -> Result<MarketClient, starfoundry_lib_market::Error> {
    MarketClient::new(SERVICE_NAME.into())
}

#[cfg(test)]
mod test_util;
#[cfg(test)]
use crate::test_util::EveGatewayTestApiClient;
use axum_server::tls_rustls::RustlsConfig;
#[cfg(test)]
pub fn eve_gateway_api_client() -> Result<EveGatewayTestApiClient, starfoundry_lib_eve_gateway::Error> {
    Ok(EveGatewayTestApiClient {})
}
#[cfg(test)]
use crate::test_util::MarketTestApiClient;
#[cfg(test)]
pub fn market_api_client() -> Result<MarketTestApiClient, starfoundry_lib_market::Error> {
    Ok(MarketTestApiClient {})
}
