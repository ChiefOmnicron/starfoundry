use prometheus_client::encoding::EncodeLabelSet;
use prometheus_client::metrics::counter::Counter;
use prometheus_client::metrics::family::Family;
use prometheus_client::metrics::histogram::Histogram;
use prometheus_client::registry::Registry;
use starfoundry_libs_types::StationId;
use uuid::Uuid;

use crate::task::WorkerTask;

const BUCKETS: [f64; 2] = [
    0.95, 0.99,
];

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct MetricLabel {
    pub worker_id: String,
    pub task: WorkerTask,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct MarketOrderLabel {
    pub station_id: i64,
}

#[derive(Clone, Debug)]
pub struct Metric {
    task_counter: Family<MetricLabel, Counter>,
    task_error_counter: Family<MetricLabel, Counter>,
    task_duration: Family<MetricLabel, Histogram>,

    market_station_rows_changed: Family<MarketOrderLabel, Counter>,
    market_station_rows_deleted: Family<MarketOrderLabel, Counter>,
    market_station_update_duration: Family<MarketOrderLabel, Histogram>,
    market_station_delete_duration: Family<MarketOrderLabel, Histogram>,

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
                Histogram::new(BUCKETS.into_iter())
            }),

            market_station_rows_changed: Family::<MarketOrderLabel, Counter>::default(),
            market_station_rows_deleted: Family::<MarketOrderLabel, Counter>::default(),
            market_station_update_duration: Family::new_with_constructor(|| {
                Histogram::new(BUCKETS.into_iter())
            }),
            market_station_delete_duration: Family::new_with_constructor(|| {
                Histogram::new(BUCKETS.into_iter())
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

        registry.register(
            "market_station_rows_changed",
            "Number of row changes per station",
            self.market_station_rows_changed.clone(),
        );

        registry.register(
            "market_station_rows_deleted",
            "Number of rows deleted per station",
            self.market_station_rows_deleted.clone(),
        );

        registry.register(
            "market_station_update_duration",
            "Time it took until all updates where done",
            self.market_station_update_duration.clone(),
        );

        registry.register(
            "market_station_delete_duration",
            "Time it took until all deletes where done",
            self.market_station_delete_duration.clone(),
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

    pub fn increase_market_order_rows_changed(
        &self,
        station_id: StationId,
        amount:     u64,
    ) {
        self.market_station_rows_changed.get_or_create(
            &&MarketOrderLabel {
                station_id: *station_id,
            }
        ).inc_by(amount);
    }

    pub fn increase_market_order_rows_deleted(
        &self,
        station_id: StationId,
        amount:     u64,
    ) {
        self.market_station_rows_changed.get_or_create(
            &&MarketOrderLabel {
                station_id: *station_id,
            }
        ).inc_by(amount);
    }

    pub fn add_market_order_latest_update_duration(
        &self,
        station_id: StationId,
        duration:   u128,
    ) {
        self.market_station_update_duration.get_or_create(
            &&MarketOrderLabel {
                station_id: *station_id,
            }
        ).observe(duration as f64);
    }

    pub fn add_market_order_latest_delete_duration(
        &self,
        station_id: StationId,
        duration:   u128,
    ) {
        self.market_station_update_duration.get_or_create(
            &&MarketOrderLabel {
                station_id: *station_id,
            }
        ).observe(duration as f64);
    }
}
