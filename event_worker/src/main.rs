#![feature(duration_constructors)]

mod api;
mod asset;
mod cleanup;
mod error;
mod industry;
mod market;
mod metric;
mod sde;
mod stock;
mod task;
mod utils;
mod worker;

use api::api;
use metric::Metric;
use prometheus_client::registry::Registry;
use sqlx::postgres::PgPoolOptions;
use starfoundry_libs_eve_api::CredentialCache;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use task::{fetch_task, TaskStatus, WorkerTask};
use tokio::time::sleep;
use tracing_subscriber::EnvFilter;

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

    let pg_addr = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL ENV not set");
    let pool = PgPoolOptions::new()
        .min_connections(10)
        .connect(&pg_addr)
        .await?;

    let worker_id = worker::register_worker(&pool).await?;
    let metrics = Metric::new(
        worker_id,
    );
    let mut registry = Registry::with_prefix("starfoundry_event_worker");
    metrics.register(&mut registry);

    let credential_cache = CredentialCache::load_from_database(&pool.clone()).await?;
    let credential_cache = Arc::new(Mutex::new(credential_cache));

    let pool_clone = pool.clone();
    tokio::task::spawn(async move {
        worker::background_task(pool_clone, worker_id).await
    });

    let pool_clone = pool.clone();
    tokio::task::spawn(async move {
        api(pool_clone, registry).await
    });

    loop {
        match fetch_task(&pool, &worker_id).await {
            Err(e) => {
                tracing::error!("error while fetching task, {}", e);
            },
            Ok(Some(mut x)) => {
                let start = std::time::Instant::now();

                tracing::info!("new task {:?}", x.task);

                let result = if cfg!(feature = "appraisal") {
                    match x.task {
                        WorkerTask::CleanupCheck => {
                            cleanup::cleanup_check::task(
                                &mut x,
                                &pool,
                            ).await
                        },
                        WorkerTask::CleanupAppraisals => {
                            cleanup::cleanup_appraisal::task(
                                &mut x,
                                &pool,
                            ).await
                        },
                        _ => Ok(()),
                    }
                } else {
                    match x.task {
                        // asset
                        WorkerTask::AssetCheck => {
                            asset::asset_check::task(
                                &mut x,
                                &pool,
                            ).await
                        },
                        WorkerTask::AssetCharacterBlueprints => {
                            asset::asset_character_blueprints::task(
                                &mut x,
                                &pool,
                                &credential_cache,
                            ).await
                        },
                        WorkerTask::AssetCorporationBlueprints => {
                            asset::asset_corporation_blueprints::task(
                                &mut x,
                                &pool,
                                &credential_cache,
                            ).await
                        },
    
                        // cleanup
                        WorkerTask::CleanupCheck => {
                            cleanup::cleanup_check::task(
                                &mut x,
                                &pool,
                            ).await
                        },
                        WorkerTask::CleanupAppraisals => {
                            cleanup::cleanup_appraisal::task(
                                &mut x,
                                &pool,
                            ).await
                        },
                        WorkerTask::CleanupSelf => {
                            cleanup::cleanup_self::task(
                                &mut x,
                                &pool,
                            ).await
                        },
                        WorkerTask::CleanupIndustryIndex => {
                            cleanup::industry_index::task(
                                &mut x,
                                &pool,
                            ).await
                        },
    
                        // industry
                        WorkerTask::IndustryCheck => {
                            industry::job_check::task(
                                &mut x,
                                &pool,
                            ).await
                        },
                        WorkerTask::IndustryJobsCharacter => {
                            industry::job_character::task(
                                &mut x,
                                &pool,
                                &credential_cache,
                            ).await
                        },
                        WorkerTask::IndustryJobsCorporation => {
                            industry::job_corporation::task(
                                &mut x,
                                &pool,
                                &credential_cache,
                            ).await
                        },
                        WorkerTask::IndustryIndex => {
                            industry::industry_index::task(
                                &mut x,
                                &pool,
                                &credential_cache,
                            ).await
                        },
    
                        // market
                        WorkerTask::MarketCheck => {
                            market::market_check::task(
                                &mut x,
                                &pool
                            )
                            .await
                        },
                        WorkerTask::MarketLatestPlayer => {
                            market::market_player_latest::task(
                                &mut x,
                                &pool,
                                &credential_cache
                            )
                            .await
                        },
                        WorkerTask::MarketLatestNpc => {
                            market::market_npc_latest::task(
                                &mut x,
                                &pool,
                                &credential_cache,
                            )
                            .await
                        },
                        WorkerTask::MarketPrices => {
                            market::market_prices::task(
                                &mut x,
                                &pool,
                                &credential_cache,
                            )
                            .await
                        },
    
                        // sde
                        WorkerTask::SdeCheck => {
                            sde::sde_check::task(
                                &mut x,
                                &pool
                            ).await
                        },
                        WorkerTask::SdeDownload => {
                            sde::download::task(
                                &mut x,
                                &pool
                            ).await
                        },
    
                        // stock
                        WorkerTask::StockCheck => {
                            stock::stock_check::task(
                                &mut x,
                                &pool,
                            ).await
                        },
                        WorkerTask::StockBlueprint => {
                            stock::blueprints::task(
                                &mut x,
                                &pool,
                                &credential_cache,
                            ).await
                        },
                    }
                };

                let end = start.elapsed().as_millis();
                metrics.increase_task_counter(x.task);
                metrics.add_task_duration(x.task, end as f64);

                if let Err(e) = result {
                    tracing::error!("{}", e);
                    metrics.increase_task_error(x.task);
                    x.add_error(e.to_string());
                    x.finish(&pool, TaskStatus::Error).await?;
                    continue;
                }

                x.finish(&pool, TaskStatus::Done).await?;
                continue;
            },
            _ => {
                tracing::info!("no new tasks, waiting");
                ()
            },
        };

        // sleep for 30 seconds until the next try to find a task
        sleep(Duration::from_secs(30)).await;
    }
}
