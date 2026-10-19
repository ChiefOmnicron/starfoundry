use std::sync::Arc;

use axum::{Router, middleware};
use prometheus_client::registry::Registry;
use sqlx::postgres::PgPoolOptions;
use starfoundry_industry::api_docs::ApiDoc;
use starfoundry_industry::config::Config;
use starfoundry_industry::metrics::{Metric, path_metrics};
use starfoundry_industry::{
    AppState, filter, healthcheck, industry, industry_hub, job_assignments, metrics, project, project_group, structure, tag
};
use tokio::select;
use tower::ServiceBuilder;
use tracing_subscriber::EnvFilter;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_scalar::{Scalar, Servable};

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
    sqlx::migrate!()
        .run(&pool)
        .await?;

    let mut metric_registry = Registry::with_prefix("starfoundry_industry_api");
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
    // build our application with a route
    let (router, api) =
        OpenApiRouter::with_openapi(ApiDoc::openapi())
            .nest("/filters", filter::routes())
            .nest("/projects", project::routes(state.clone()))
            .nest("/project-groups", project_group::routes(state.clone()))
            .nest("/structures", structure::routes(state.clone()))
            .nest("/industry", industry::routes())
            .nest("/industry-hubs", industry_hub::routes(state.clone()))
            .nest("/job-assignments", job_assignments::routes())
            .nest("/tags", tag::routes())
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
