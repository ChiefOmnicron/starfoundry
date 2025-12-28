use prometheus_client::encoding::EncodeLabelSet;
use prometheus_client::metrics::counter::Counter;
use prometheus_client::metrics::family::Family;
use prometheus_client::metrics::histogram::Histogram;
use prometheus_client::registry::Registry;
use starfoundry_lib_types::StructureId;
use starfoundry_lib_worker::TaskMetric;

const BUCKETS: [f64; 2] = [
    0.95, 0.99,
];

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct MarketOrderLabel {
    pub structure_id: i64,
}

#[derive(Clone, Debug)]
pub struct WorkerMetric {
    market_station_rows_changed:    Family<MarketOrderLabel, Counter>,
    market_station_rows_deleted:    Family<MarketOrderLabel, Counter>,
    market_station_update_duration: Family<MarketOrderLabel, Histogram>,
    market_station_delete_duration: Family<MarketOrderLabel, Histogram>,
}

impl WorkerMetric {
    pub fn new() -> Self {
        Self {
            market_station_rows_changed: Family::<MarketOrderLabel, Counter>::default(),
            market_station_rows_deleted: Family::<MarketOrderLabel, Counter>::default(),
            market_station_update_duration: Family::new_with_constructor(|| {
                Histogram::new(BUCKETS.into_iter())
            }),
            market_station_delete_duration: Family::new_with_constructor(|| {
                Histogram::new(BUCKETS.into_iter())
            }),
        }
    }

    pub fn increase_market_order_rows_changed(
        &self,
        station_id: StructureId,
        amount:     u64,
    ) {
        self.market_station_rows_changed.get_or_create(
            &&MarketOrderLabel {
                structure_id: *station_id,
            }
        ).inc_by(amount);
    }

    pub fn increase_market_order_rows_deleted(
        &self,
        station_id: StructureId,
        amount:     u64,
    ) {
        self.market_station_rows_deleted.get_or_create(
            &&MarketOrderLabel {
                structure_id: *station_id,
            }
        ).inc_by(amount);
    }

    pub fn add_market_order_latest_update_duration(
        &self,
        station_id: StructureId,
        duration:   u128,
    ) {
        self.market_station_update_duration.get_or_create(
            &&MarketOrderLabel {
                structure_id: *station_id,
            }
        ).observe(duration as f64);
    }

    pub fn add_market_order_latest_delete_duration(
        &self,
        station_id: StructureId,
        duration:   u128,
    ) {
        self.market_station_update_duration.get_or_create(
            &&MarketOrderLabel {
                structure_id: *station_id,
            }
        ).observe(duration as f64);
    }
}

impl TaskMetric for WorkerMetric {
    fn register(
        &self,
        registry: &mut Registry,
    ) {
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
}
