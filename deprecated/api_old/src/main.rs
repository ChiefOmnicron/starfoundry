use prometheus_client::registry::Registry;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use starfoundry_bin_api_old::*;
use starfoundry_bin_api_old::config::Config;
use starfoundry_bin_api_old::metric::Metric;
use starfoundry_lib_eve_api::CredentialCache;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tracing_subscriber::EnvFilter;
use utoipa::OpenApi;
use warp::Filter;

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

    let config = Config::load();

    let pool = PgPoolOptions::new()
        .connect(&config.database_url)
        .await?;

    tracing::info!("Starting server");

    let credential_cache = CredentialCache::load_from_database(&pool.clone()).await?;
    let credential_cache = Arc::new(Mutex::new(credential_cache));

    let server = Server::new(
        pool,
        credential_cache,
    );

    server.listen(&config.server_address).await;

    Ok(())
}

struct Server {
    pool:             PgPool,
    credential_cache: Arc<Mutex<CredentialCache>>,
}

impl Server {
    pub fn new(
        pool:             PgPool,
        credential_cache: Arc<Mutex<CredentialCache>>,
    ) -> Self {
        Self { pool, credential_cache }
    }

    pub async fn listen(
        self,
        server_address: &SocketAddr,
    ) {
        let metrics = Metric::new();
        let mut registry = Registry::with_prefix("starfoundry");
        metrics.register(&mut registry);
        let registry = Arc::new(registry);
        let metric = Arc::new(metrics);

        let api_doc = warp::path::end()
            .and(warp::get())
            .map(|| warp::reply::html(include_str!("api.html")));
        let definition = warp::path!("definition")
            .and(warp::get())
            .map(|| warp::reply::json(&crate::api_docs::ApiDoc::openapi()));

        let base_path = warp::any().boxed();
        let base_path_v1 = warp::path!("v1" / ..).boxed();

        let appraisal               = appraisal::api(self.pool.clone(), metric.clone(), base_path.clone());
        let auth                    = auth::api(self.pool.clone(), base_path.clone(), self.credential_cache.clone());
        let characters              = character::api(self.pool.clone(), base_path.clone(), self.credential_cache.clone());
        let corporations            = corporation::api(self.pool.clone(), base_path.clone(), self.credential_cache.clone());
        let indy                    = industry::api(self.pool.clone(), base_path.clone(), self.credential_cache.clone());
        let item                    = item::api(self.pool.clone(), base_path.clone());
        let job_detection           = job_detection::api(self.pool.clone(), base_path.clone(), self.credential_cache.clone());
        let notifications           = notification::api(self.pool.clone(), base_path.clone(), self.credential_cache.clone());
        let projects                = project::api(self.pool.clone(), base_path.clone(), self.credential_cache.clone());
        let project_groups          = project_group::api(self.pool.clone(), base_path.clone(), self.credential_cache.clone());
        let stock                   = stock::api(self.pool.clone(), base_path.clone(), self.credential_cache.clone());
        let search                  = search::api(self.pool.clone(), base_path.clone());
        let structure               = structure::api(self.pool.clone(), base_path.clone(), self.credential_cache.clone());
        let structure_dynamic_group = structure_dynamic_group::api(self.pool.clone(), base_path.clone(), self.credential_cache.clone());
        let structure_group         = structure_group::api(self.pool.clone(), base_path.clone(), self.credential_cache.clone());
        let version                 = version::api(base_path.clone());

        let special_routes = crate::healthcheck::api(self.pool.clone())
            .or(crate::metric::api(registry.clone(), metric.clone()))
            .or(version);

        if cfg!(feature = "appraisal") {
            let base = crate::healthcheck::api(self.pool.clone())
                .or(appraisal)
                .or(api_doc)
                .or(definition);

            let v1 = base_path_v1.and(base.clone());
            let routes = base
                .or(v1)
                .or(special_routes)
                .recover(crate::rejection::handle_rejection)
                .with(warp::wrap_fn(|f| metric_wrapper(f, metric.clone())));

            warp::serve(routes)
                .run(*server_address)
                .await;
        } else {
            let base = crate::feature_flags::api(base_path.clone())
                .or(appraisal)
                .or(api_doc)
                .or(definition)
                .or(auth)
                .or(characters)
                .or(corporations)
                .or(indy)
                .or(item)
                .or(job_detection)
                .or(notifications)
                .or(projects)
                .or(project_groups)
                .or(search)
                .or(stock)
                .or(structure)
                .or(structure_dynamic_group)
                .or(structure_group);

            let v1 = base_path_v1.and(base.clone());
            let routes = base
                .or(v1)
                .or(special_routes)
                .recover(crate::rejection::handle_rejection)
                .with(warp::wrap_fn(|f| metric_wrapper(f, metric.clone())));

            warp::serve(routes)
                .run(*server_address)
                .await;
        }
    }
}
