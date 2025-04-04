use handlebars::Handlebars;
use serde_json::json;
use serde::Serialize;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tracing_subscriber::EnvFilter;
use warp::Filter;

use crate::config::Config;
use starfoundry_libs_appraisal::internal::fetch;
use sqlx::PgPool;
use std::convert::Infallible;
use num_format::{Locale, ToFormattedString};

mod config;

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

    let template = r#"<!DOCTYPE html>
        <html>
        <head>
            <title>Appraisal {{code}}: Buy {{ buy }} / Sell {{ sell }}</title>

            <meta name="description" content="{{ description }}">
        </head>
        </html>"#;

    let mut hb = Handlebars::new();
    // register the template
    hb.register_template_string("template.html", template).unwrap();
    let hb = Arc::new(hb);

    let handlebars = move |with_template| render(with_template, hb.clone());

    let route = warp::get()
        .and(with_pool(pool.clone()))
        //.and(with_hb(hb.clone()))
        .and(warp::path!("appraisal" / String))
        //.then(|pool: PgPool, hb: Arc<Handlebars>, code: String| async move {
        .then(|pool: PgPool, code: String| async move {
            let data = fetch(
                    &pool,
                    code.clone(),
                )
                .await
                .unwrap()
                .unwrap();
            let items = data
                .items
                .iter()
                .map(|x| {
                    format!("{} - {}", x.meta.name, x.quantity.to_formatted_string(&Locale::en))
                })
                .collect::<Vec<_>>()
                .join("\n");

            let buy= data
                .items
                .iter()
                .map(|x| x.buy.max)
                .sum::<f64>()
                .round();
            let buy = if buy >= 1_000_000_000f64 {
                format!("{:.2} Billion", buy / 1_000_000_000f64)
            } else if buy >= 1_000_000f64 {
                format!("{:.2} Million", buy / 1_000_000f64)
            } else if buy >= 1_000_000f64 {
                format!("{:.2} Thousand", buy / 1_000f64)
            } else {
                format!("{:.2}", buy)
            };

            let sell = data
                .items
                .iter()
                .map(|x| x.sell.min)
                .sum::<f64>()
                .round();
            let sell = if sell >= 1_000_000_000f64 {
                format!("{:.2} Billion", sell / 1_000_000_000f64)
            } else if sell >= 1_000_000f64 {
                format!("{:.2} Million", sell / 1_000_000f64)
            } else if sell >= 1_000_000f64 {
                format!("{:.2} Thousand", sell / 1_000f64)
            } else {
                format!("{:.2}", sell)
            };

            WithTemplate {
                name: "template.html",
                value: json!({
                    "code": code,
                    "description" : items,
                    "buy": buy,
                    "sell": sell,
                }),
            }
        })
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

struct WithTemplate<T: Serialize> {
    name: &'static str,
    value: T,
}

fn with_pool(
    pool: PgPool,
) -> impl Filter<Extract = (PgPool,), Error = Infallible> + Clone {
    warp::any().map(move || pool.clone())
}
