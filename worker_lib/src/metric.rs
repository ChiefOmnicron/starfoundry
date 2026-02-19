use prometheus_client::metrics::family::Family;
use prometheus_client::metrics::counter::Counter;
use prometheus_client::encoding::EncodeLabelSet;
use prometheus_client::metrics::histogram::Histogram;
use prometheus_client::registry::Registry;
use uuid::Uuid;

use crate::TaskStatus;

const TOTAL_TASK_DURATION_BUCKETS: [f64; 6] = [
    // times in ms
    500f64, 1_000f64, 30_000f64, 60_000f64, 300_000f64, 600_000f64,
];

pub trait TaskMetric: Clone {
    fn register(
        &self,
        registry: &mut Registry,
    );
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct MetricLabel {
    pub worker_id: String,
    pub task:      String,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct MetricLabelStatus {
    pub worker_id: String,
    pub task:      String,
    pub status:    String,
}

#[derive(Clone)]
pub struct InternalMetric {
    task_counter:  Family<MetricLabelStatus, Counter>,
    task_duration: Family<MetricLabel, Histogram>,

    worker_id:     Uuid,
}

impl InternalMetric {
    pub fn new(
        worker_id: Uuid,
    ) -> Self {
        Self {
            task_counter:  Family::<MetricLabelStatus, Counter>::default(),
            task_duration: Family::new_with_constructor(|| {
                Histogram::new(TOTAL_TASK_DURATION_BUCKETS.into_iter())
            }),

            worker_id,
        }
    }

    pub fn add_task_duration(
        &self,
        task:     String,
        duration: u128,
    ) {
        self.task_duration.get_or_create(
            &MetricLabel {
                worker_id: self.worker_id.to_string(),
                task,
            }
        ).observe(duration as f64);
    }

    pub fn increase_task_counter(
        &self,
        task:   String,
        status: TaskStatus,
    ) {
        self.task_counter.get_or_create(
            &MetricLabelStatus {
                worker_id: self.worker_id.to_string(),
                task:      task,
                status:    status.into(),
            }
        ).inc();
    }
}

impl TaskMetric for InternalMetric {
    fn register(
        &self,
        registry: &mut Registry,
    ) {
        registry.register(
            "task_durations",
            "Tracks the time it takes to finish a task",
            self.task_duration.clone(),
        );

        registry.register(
            "tasks_counter",
            "Total number of tasks finished, split by their resulting status",
            self.task_counter.clone(),
        );
    }
}
