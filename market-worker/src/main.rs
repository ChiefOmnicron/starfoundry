// Avoid musl's default allocator due to lackluster performance
// https://nickb.dev/blog/default-musl-allocator-considered-harmful-to-performance
#[cfg(target_env = "musl")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

mod by_npc_station;
mod by_player_station;
mod by_region;
mod error;
mod insert;
mod metric;
mod tasks;

use std::time::Duration;

use sqlx::postgres::PgPoolOptions;
use starfoundry_lib_worker::{Task, TaskStatus, Worker};

use tokio::sync::mpsc;
use tokio::time;
use tracing_subscriber::EnvFilter;

pub use self::tasks::*;

use prometheus_client::registry::Registry;
use sqlx::PgPool;

use self::by_npc_station::by_npc_station;
use self::by_player_station::by_player_station;

use crate::error::Error;
use crate::metric::WorkerMetric;
use crate::by_region::by_region;

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

    // 15 Minutes
    let task_timeout = time::sleep(Duration::from_secs(60 * 15));
    tokio::pin!(task_timeout);

    let pool = PgPoolOptions::new()
        .connect("postgresql://postgres:postgres@localhost:5432/dev-sf-market")
        .await?;

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
        worker.run(metric_registry)
    );

    while let Some(task) = rx.recv().await {
        let mut task = task.clone();
        tokio::select! {
            status = task_select(
                &pool,
                &mut task
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
                tracing::error!("Task timeout");
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
    pool: &PgPool,
    task: &mut Task<WorkerMetric, WorkerMarketTask>,
) -> Result<(), Error> {
    match task.task {
        WorkerMarketTask::Sync         => {
            dummy(
                pool,
                task,
            ).await
        },
        WorkerMarketTask::LatestNpc    => {
            by_npc_station(
                pool,
                task,
            ).await
        },
        WorkerMarketTask::LatestPlayer => {
            by_player_station(
                pool,
                task,
            ).await
        },
        WorkerMarketTask::LatestRegion => {
            by_region(
                pool,
                task,
            ).await
        },
        WorkerMarketTask::Prices       => {
            dummy(
                pool,
                task,
            ).await
        },
    }
}

async fn dummy(
    _pool: &PgPool,
    _task: &mut Task<WorkerMetric, WorkerMarketTask>,
) -> Result<(), Error> {
    Ok(())
}
