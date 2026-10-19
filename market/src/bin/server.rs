use std::sync::Arc;

use axum::{Router, middleware};
use prometheus_client::registry::Registry;
use sqlx::postgres::PgPoolOptions;
use starfoundry_market::api_docs::ApiDoc;
use starfoundry_market::config::Config;
use starfoundry_market::metrics::{Metric, path_metrics};
use starfoundry_market::{
    AppState, healthcheck, market, metrics, price, structure
};
use tokio::select;
use tower::ServiceBuilder;
use tracing_subscriber::EnvFilter;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_scalar::{Scalar, Servable};

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
    sqlx::migrate!()
        .run(&pool)
        .await?;

    let mut metric_registry = Registry::with_prefix("starfoundry_market_api");
    let metric = Metric::new();
    metric.register(&mut metric_registry);

    let state = AppState {
        postgres: pool,
        metric:   Arc::new(metric),
    };

    tracing::info!(
        "Starting app server on {}",
        config
            .app_address
            .local_addr()
            .unwrap()
    );
    tracing::info!(
        "Starting service server on {}",
        config
            .service_address
            .local_addr()
            .unwrap()
    );

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

    Ok(())
}

fn app(state: AppState) -> Router {
    // TODO: add market route with all markets the user has access to

    // build our application with a route
    let (router, api) =
        OpenApiRouter::with_openapi(ApiDoc::openapi())
            .nest("/markets", market::routes())
            .nest("/prices", price::routes())
            .nest("/structures", structure::routes())
            .layer(ServiceBuilder::new().layer(middleware::from_fn_with_state(
                state.clone(),
                path_metrics,
            )))
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
    registry: Arc<Registry>,
) -> Router {
    Router::new()
        .nest("/health", healthcheck::routes().with_state(state))
        .route(
            "/metrics",
            axum::routing::get(|| async move { metrics::route(registry) }),
        )
}
