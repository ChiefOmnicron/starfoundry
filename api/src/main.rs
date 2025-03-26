use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use starfoundry_bin_api::*;
use starfoundry_bin_api::config::Config;
use starfoundry_libs_eve_api::CredentialCache;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tracing_subscriber::EnvFilter;
use utoipa::OpenApi;
use warp::Filter;
use warp::http::StatusCode;
use warp::reject::Rejection;
use warp::reply::Reply;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
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
        let api_doc = warp::path!("api" / "v1")
            .and(warp::get())
            .map(|| {
                if cfg!(debug_assertions) {
                    warp::reply::html(include_str!("api-local.html"))
                } else {
                    warp::reply::html(include_str!("api.html"))
                }
            });
        let definition = warp::path!("api" / "v1" / "definition")
            .and(warp::get())
            .map(|| warp::reply::json(&crate::api_docs::ApiDoc::openapi()));

        let base_path = warp::path!("api" / "v1" / ..).boxed();

        let appraisal               = appraisal::api(self.pool.clone(), base_path.clone());
        let auth                    = auth::api(self.pool.clone(), base_path.clone(), self.credential_cache.clone());
        let characters              = character::api(self.pool.clone(), base_path.clone(), self.credential_cache.clone());
        let corporations            = corporation::api(self.pool.clone(), base_path.clone(), self.credential_cache.clone());
        let indy                    = industry::api(self.pool.clone(), base_path.clone(), self.credential_cache.clone());
        let item                    = item::api(self.pool.clone(), base_path.clone(), self.credential_cache.clone());
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

        if cfg!(feature = "appraisal") {
            let routes = crate::healthcheck::api(self.pool.clone())
                .or(crate::feature_flags::api(base_path.clone()))
                .or(appraisal)
                .or(api_doc)
                .or(definition)
                .or(version)
                .recover(handle_rejection);
                //.with(warp::wrap_fn(metric_wrapper));

            warp::serve(routes)
                .run(*server_address)
                .await;
        } else {
            let routes = crate::healthcheck::api(self.pool.clone())
                .or(crate::feature_flags::api(base_path.clone()))
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
                .or(structure_group)
                .or(version)
                .recover(handle_rejection);
                //.with(warp::wrap_fn(metric_wrapper));

            warp::serve(routes)
                .run(*server_address)
                .await;
        }
    }
}

async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let mut json = warp::reply::json(&());

    if let Some(ReplyError::Forbidden) = err.find() {
        code = StatusCode::NOT_FOUND;
    } else if let Some(ReplyError::BadRequest) = err.find() {
        code = StatusCode::BAD_REQUEST;
    } else if let Some(ReplyError::BadRequestWithPayload(x)) = err.find() {
        code = StatusCode::BAD_REQUEST;
        json = warp::reply::json(&x);
    } else if let Some(ReplyError::Unauthorized) = err.find() {
        code = StatusCode::UNAUTHORIZED;
    } else if let Some(ReplyError::Forbidden) = err.find() {
        code = StatusCode::FORBIDDEN;
    } else if let Some(ReplyError::Forbidden) = err.find() {
        code = StatusCode::NOT_FOUND;
    } else if let Some(ReplyError::Internal) = err.find() {
        code = StatusCode::INTERNAL_SERVER_ERROR;
    } else if let Some(ReplyError::NotFound) = err.find() {
        code = StatusCode::NOT_FOUND;
    } else {
        tracing::error!("Unhandled error {:?}", err);
        code = StatusCode::UNAUTHORIZED;
    }

    Ok(warp::reply::with_status(json, code))
}
