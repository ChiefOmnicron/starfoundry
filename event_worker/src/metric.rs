use prometheus_client::encoding::EncodeLabelSet;
use prometheus_client::metrics::counter::Counter;
use prometheus_client::metrics::family::Family;
use prometheus_client::metrics::histogram::Histogram;
use prometheus_client::registry::Registry;
use uuid::Uuid;

use crate::task::WorkerTask;

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct MetricLabel {
    pub worker_id: String,
    pub task: WorkerTask,
}

#[derive(Clone, Debug)]
pub struct Metric {
    task_counter: Family<MetricLabel, Counter>,
    task_error_counter: Family<MetricLabel, Counter>,
    task_duration: Family<MetricLabel, Histogram>,

    worker_id: Uuid,
}

impl Metric {
    pub fn new(
        worker_id: Uuid,
    ) -> Self {
        Self {
            task_counter: Family::<MetricLabel, Counter>::default(),
            task_error_counter: Family::<MetricLabel, Counter>::default(),
            task_duration: Family::new_with_constructor(|| {
                Histogram::new(vec![0.95f64, 0.99f64].into_iter())
            }),

            worker_id,
        }
    }

    pub fn register(
        &self,
        registry: &mut Registry,
    ) {
        registry.register(
            "task_durations",
            "Tracks the time it takes to finish a task",
            self.task_duration.clone(),
        );

        registry.register(
            "tasks_processed",
            "Total number of tasks finished, includes those that had an error",
            self.task_counter.clone(),
        );

        registry.register(
            "task_errors",
            "Counts the number of errors",
            self.task_error_counter.clone(),
        );
    }

    pub fn add_task_duration(
        &self,
        task: WorkerTask,
        duration: f64,
    ) {
        self.task_duration.get_or_create(
            &MetricLabel {
                worker_id: self.worker_id.to_string(),
                task,
            }
        ).observe(duration);
    }

    pub fn increase_task_counter(
        &self,
        task: WorkerTask,
    ) {
        self.task_counter.get_or_create(
            &MetricLabel {
                worker_id: self.worker_id.to_string(),
                task,
            }
        ).inc();
    }

    pub fn increase_task_error(
        &self,
        task: WorkerTask,
    ) {
        self.task_error_counter.get_or_create(
            &MetricLabel {
                worker_id: self.worker_id.to_string(),
                task,
            }
        ).inc();
    }
}
