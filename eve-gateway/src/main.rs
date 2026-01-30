// Avoid musl's default allocator due to lackluster performance
// https://nickb.dev/blog/default-musl-allocator-considered-harmful-to-performance
#[cfg(target_env = "musl")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use axum_server::tls_rustls::RustlsConfig;
use axum::{middleware, Router};
use sqlx::postgres::PgPoolOptions;
use starfoundry_bin_eve_gateway::{auth, character, contract, corporation, healthcheck, industry, internal, item, market, search, structure, universe};
use starfoundry_bin_eve_gateway::api_docs::ApiDoc;
use starfoundry_bin_eve_gateway::config::Config;
use starfoundry_bin_eve_gateway::metrics::{self, path_metrics, setup_metrics_recorder};
use starfoundry_bin_eve_gateway::state::AppState;
use std::sync::Arc;
use tokio::select;
use tower::ServiceBuilder;
use tracing_subscriber::EnvFilter;
use utoipa_axum::router::OpenApiRouter;
use utoipa_scalar::{Scalar, Servable};
use utoipa::OpenApi;

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
    sqlx::migrate!().run(&postgres).await?;

    let state = AppState {
        postgres,
        auth_domains: Arc::new(config.domains),
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
        .nest("/contracts", contract::routes())
        .nest("/corporations", corporation::routes())
        .nest("/industry", industry::routes())
        .nest("/items", item::routes())
        .nest("/market", market::routes())
        .nest("/search", search::routes())
        .nest("/structures", structure::routes())
        .nest("/universe", universe::routes())
        .nest("/internal", internal::routes())
        .layer(
            ServiceBuilder::new().layer(middleware::from_fn(path_metrics))
        )
        .with_state(state.clone())
        .split_for_parts();

    let router = router.merge(Scalar::with_url("/", api));

    //let router_v1 = Router::new().nest("/v1", router.clone());
    //let router_latest = Router::new().nest("/latest", router.clone());

    router
        //.merge(router_v1)
        //.merge(router_latest)
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
