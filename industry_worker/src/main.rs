mod config;
mod error;
mod jobs;
mod metric;
mod sync;
mod tasks;

use prometheus_client::registry::Registry;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use starfoundry_lib_worker::{Task, TaskStatus, Worker};
use std::time::Duration;
use tokio::sync::mpsc;
use tracing_subscriber::EnvFilter;

pub use self::tasks::*;

use self::sync::{sync, sync_task};

use crate::config::Config;
use crate::error::Result;
use crate::metric::WorkerMetric;
use crate::jobs::corporation_jobs;

pub const SERVICE_NAME: &str = "SF_INDUSTRY_WORKER";

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

    let mut metric_registry = Registry::with_prefix("starfoundry_industry_worker");
    let metric = WorkerMetric::new();

    let (tx, mut rx) = mpsc::channel(5);

    let worker: Worker<WorkerMetric, WorkerIndustryTask> = Worker::init(
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
            ) => {
                let pool = pool.clone();

                match status {
                    Ok(_) => {
                        task.finish(
                            &pool,
                            TaskStatus::Done,
                        )
                        .await
                        .unwrap();
                    },
                    Err(e) => {
                        task.append_error(e.to_string());
                        task.finish(
                            &pool,
                            TaskStatus::Error,
                        )
                        .await
                        .unwrap();
                    }
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
    task:   &mut Task<WorkerMetric, WorkerIndustryTask>,
) -> Result<()> {
    match task.task {
        WorkerIndustryTask::Sync            => {
            sync_task(
                pool,
                task,
            ).await
        },
        WorkerIndustryTask::JobCharacter    => {
            unimplemented!()
        },
        WorkerIndustryTask::JobCorporation  => {
            corporation_jobs(
                    pool,
                    task,
                )
                .await
        },
    }
}
