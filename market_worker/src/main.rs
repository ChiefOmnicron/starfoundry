mod config;
mod contract;
mod error;
mod metric;
mod order;
mod prices;
mod sync;
mod tasks;

use prometheus_client::registry::Registry;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use starfoundry_lib_worker::{Task, TaskStatus, Worker, cleanup_task};
use std::time::Duration;
use tokio::sync::mpsc;
use tracing_subscriber::EnvFilter;

pub use self::tasks::*;

use self::prices::prices;
use self::sync::{sync, sync_task};

use crate::config::Config;
use crate::contract::*;
use crate::error::Result;
use crate::metric::WorkerMetric;
use crate::order::*;

pub const SERVICE_NAME: &str = "SF_MARKET_WORKER";

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
    sqlx::migrate!().run(&pool).await?;

    let mut metric_registry = Registry::with_prefix("starfoundry_market_worker");
    let metric = WorkerMetric::new();

    let (tx, mut rx) = mpsc::channel(5);

    let worker: Worker<WorkerMetric, WorkerMarketTask> = Worker::init(
            pool.clone(),
            tx,

            metric,
            &mut metric_registry,
        )
        .await
        .unwrap();

    tokio::task::spawn(
        worker.run(
            metric_registry,
            config.service_address,
        )
    );

    sync(
        &pool,
        config.clone(),
    ).await?;

    while let Some(task) = rx.recv().await {
        // 15 Minutes
        let task_timeout = tokio::time::sleep(Duration::from_secs(60 * 15));
        tokio::pin!(task_timeout);

        let mut task = task.clone();
        tokio::select! {
            status = task_select(
                &pool,
                &mut task,
                config.clone(),
            ) => {
                let pool = pool.clone();

                if let Ok(_) = status {
                    task.finish(
                            &pool,
                            TaskStatus::Done,
                        )
                        .await
                        .unwrap();
                } else {
                    task.finish(
                            &pool,
                            TaskStatus::Error,
                        )
                        .await
                        .unwrap();
                }
            }
            _ = &mut task_timeout => {
                tracing::warn!("Task timeout");
                task.finish(
                        &pool,
                        TaskStatus::Timeout,
                    )
                    .await
                    .unwrap();
            }
        }
    }

    Ok(())
}

async fn task_select(
    pool:   &PgPool,
    task:   &mut Task<WorkerMetric, WorkerMarketTask>,
    config: Config,
) -> Result<()> {
    match task.task {
        WorkerMarketTask::Sync                  => {
            sync_task(
                pool,
                task,
                config,
            ).await
        },
        WorkerMarketTask::Cleanup                  => {
            cleanup_task(
                    pool,
                    task,
                )
                .await
                .map_err(Into::into)
        },
        WorkerMarketTask::LatestNpc             => {
            by_npc_station_task(
                    pool,
                    task,
                )
                .await
        },
        WorkerMarketTask::LatestPlayer          => {
            by_player_station_task(
                    pool,
                    task,
                )
                .await
        },
        WorkerMarketTask::LatestRegion          => {
            by_region_task(
                    pool,
                    task,
                )
                .await
        },
        WorkerMarketTask::Prices                => {
            prices(
                    pool,
                    task,
                )
                .await
        },
        WorkerMarketTask::PublicContracts       => {
            public_contracts(
                    pool,
                    task,
                )
                .await
        },
        WorkerMarketTask::PublicContractItems   => {
            public_contract_items(
                    pool,
                    task,
                )
                .await
        },
        WorkerMarketTask::CharacterOrders       => {
            character_orders(
                    pool,
                    task,
                )
                .await
        },
        WorkerMarketTask::CorporationOrders     => {
            corporation_orders(
                    pool,
                    task,
                )
                .await
        },
    }
}

async fn _dummy(
    _pool: &PgPool,
    _task: &mut Task<WorkerMetric, WorkerMarketTask>,
) -> Result<()> {
    Ok(())
}
