mod api_docs;
mod config;
mod healthcheck;
mod lp;
mod market;
mod metrics;
mod price;
mod state;

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
const SERVICE_NAME: &str = "SF_MARKET";

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
    // TODO: add market route with all markets the user has access to

    // build our application with a route
    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/markets", market::routes(state.clone()))
        .nest("/prices", price::routes())
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

#[cfg(not(test))]
use starfoundry_lib_eve_gateway::EveGatewayClient;
#[cfg(not(test))]
pub fn eve_gateway_api_client() -> Result<EveGatewayClient, starfoundry_lib_eve_gateway::Error> {
    EveGatewayClient::new(SERVICE_NAME.into())
}

#[cfg(test)]
use starfoundry_lib_eve_gateway::EveGatewayClient;
#[cfg(test)]
pub fn eve_gateway_api_client() -> Result<EveGatewayClient, starfoundry_lib_eve_gateway::Error> {
    use starfoundry_lib_eve_gateway::EveGatewayClient;

    EveGatewayClient::new("MARKET_TESTING".into())
}
