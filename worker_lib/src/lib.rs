mod api;
mod cleanup;
mod error;
mod metric;
mod task;
mod worker;

pub use self::cleanup::*;
pub use self::error::*;
pub use self::task::*;
pub use self::metric::TaskMetric;

use std::time::Duration;

use prometheus_client::registry::Registry;
use sqlx::PgPool;
use std::net::SocketAddr;
use tokio::sync::mpsc;
use tokio::time::sleep;
use uuid::Uuid;

use crate::metric::InternalMetric;
use crate::api::api;

pub struct Worker<M: TaskMetric, WT: WorkerTask> {
    pool:        PgPool,
    self_uuid:   Uuid,
    mpsc_sender: mpsc::Sender<Task<M, WT>>,

    metric:          M,
    internal_metric: InternalMetric,
}

impl<M, WT> Worker<M, WT>
    where
        M: TaskMetric,
        WT: WorkerTask,
        <WT as TryFrom<String>>::Error: std::fmt::Debug {

    pub async fn init(
        pool:            PgPool,
        mpsc_sender:     mpsc::Sender<Task<M, WT>>,

        metric:          M,
        metric_registry: &mut Registry,
    ) -> Result<Self> {
        let worker_id = worker::register_worker(&pool).await?;

        let internal_metric = InternalMetric::new(worker_id);
        internal_metric.register(metric_registry);
        metric.register(metric_registry);

        Ok(Self {
            pool:        pool.clone(),
            self_uuid:   worker_id,
            mpsc_sender: mpsc_sender,

            metric:          metric,
            internal_metric: internal_metric,
        })
    }

    pub async fn run(
        self,
        metric_registry: Registry,
        socket_addr:     SocketAddr,
    ) -> Result<()> {
        let api = api(self.pool.clone(), metric_registry, socket_addr);

        let background_task = worker::background_task(
            self.pool.clone(),
            self.self_uuid,
        );

        let pull_task = self.pull_task();

        // TODO: handle errors
        let _ = tokio::join! {
            api,
            background_task,
            pull_task,
        };

        Ok(())
    }

    pub fn worker_id(
        &self,
    ) -> Uuid {
        self.self_uuid
    }

    async fn pull_task(
        &self,
    ) {
        loop {
            match fetch_task(
                    &self.pool,
                    &self.self_uuid,

                    self.metric.clone(),
                    self.internal_metric.clone(),
                )
                .await {

                Err(e) => {
                    tracing::error!("error while fetching task, {}", e);
                    continue;
                },
                Ok(Some(x)) => {
                    tracing::info!("new task {:?}", x.task);
                    self.mpsc_sender.send(x).await.unwrap();
                    continue;
                },
                _ => {
                    tracing::info!("no new tasks, waiting");
                    ()
                },
            }

            // sleep for 30 seconds until the next try to find a task
            sleep(Duration::from_secs(30)).await;
        }
    }
}
