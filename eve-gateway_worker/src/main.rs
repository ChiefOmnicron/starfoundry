mod asset;
mod blueprint;
mod config;
mod error;
mod industry;
mod metric;
mod tasks;
mod sync;

use std::time::Duration;
use sqlx::postgres::PgPoolOptions;
use starfoundry_lib_worker::{Task, TaskStatus, Worker};
use tokio::sync::mpsc;
use tracing_subscriber::EnvFilter;
use prometheus_client::registry::Registry;
use sqlx::PgPool;

use crate::config::Config;
use crate::error::Result;
use crate::metric::WorkerMetric;
use crate::sync::{sync, sync_task};
use crate::tasks::WorkerEveGatewayTask;

pub const SERVICE_NAME: &str = "SF_EVE_GATEWAY_WORKER";

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

    let mut metric_registry = Registry::with_prefix("starfoundry_eve_gateway_worker");
    let metric = WorkerMetric::new();

    let (tx, mut rx) = mpsc::channel(5);

    let worker: Worker<WorkerMetric, WorkerEveGatewayTask> = Worker::init(
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

    sync(&pool).await?;

    while let Some(task) = rx.recv().await {
        // 15 Minutes
        let task_timeout = tokio::time::sleep(Duration::from_secs(60 * 15));
        tokio::pin!(task_timeout);

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
    pool: &PgPool,
    task: &mut Task<WorkerMetric, WorkerEveGatewayTask>,
) -> Result<()> {
    match task.task {
        WorkerEveGatewayTask::Sync => {
            sync_task(
                pool,
                task,
            ).await
        },

        WorkerEveGatewayTask::CharacterAssets => {
            crate::asset::character::assets(
                pool,
                task,
            ).await
        },
        WorkerEveGatewayTask::CorporationAssets => {
            crate::asset::corporation::assets(
                pool,
                task,
            ).await
        },

        WorkerEveGatewayTask::CharacterBlueprints => {
            crate::blueprint::character::blueprints(
                pool,
                task,
            ).await
        },
        WorkerEveGatewayTask::CorporationBlueprints => {
            crate::blueprint::corporation::blueprints(
                pool,
                task,
            ).await
        },

        WorkerEveGatewayTask::SystemIndex => {
            crate::industry::system_index(
                pool,
                task,
            ).await
        },
        WorkerEveGatewayTask::SystemIndexCompress => {
            crate::industry::system_index_compress(
                pool,
                task,
            ).await
        },
    }
}
