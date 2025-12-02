/// Webserver for generating HTML pages that satisfy the needs of bots
/// 
/// Currently supported StarFoundry modules:
/// - Appraisal
/// 

// Avoid musl's default allocator due to lackluster performance
// https://nickb.dev/blog/default-musl-allocator-considered-harmful-to-performance
#[cfg(target_env = "musl")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

mod appraisal;
mod config;

use handlebars::Handlebars;
use serde::Serialize;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tracing_subscriber::EnvFilter;
use warp::Filter;

use sqlx::PgPool;
use std::convert::Infallible;

use crate::config::Config;

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

    let mut hb = Handlebars::new();
    hb.register_template_string("appraisal.html", appraisal::template()).unwrap();
    let hb = Arc::new(hb);

    let handlebars = move |with_template| render(with_template, hb.clone());

    let route = warp::get()
        .and(with_pool(pool.clone()))
        .and(warp::path!("appraisal" / String))
        .then(appraisal::appraisal)
        .map(handlebars);

    warp::serve(route).run(config.server_address).await;

    Ok(())
}

fn render<T>(template: WithTemplate<T>, hbs: Arc<Handlebars<'_>>) -> impl warp::Reply
where
    T: Serialize,
{
    let render = hbs
        .render(template.name, &template.value)
        .unwrap_or_else(|err| err.to_string());
    warp::reply::html(render)
}

pub struct WithTemplate<T: Serialize> {
    name: &'static str,
    value: T,
}

fn with_pool(
    pool: PgPool,
) -> impl Filter<Extract = (PgPool,), Error = Infallible> + Clone {
    warp::any().map(move || pool.clone())
}
