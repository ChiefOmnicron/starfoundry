use prometheus_client::encoding::EncodeLabelSet;
use prometheus_client::metrics::family::Family;
use prometheus_client::metrics::histogram::Histogram;
use prometheus_client::registry::Registry;
use starfoundry_lib_worker::TaskMetric;
use prometheus_client::metrics::gauge::Gauge;

const BUCKETS: [f64; 2] = [
    0.95, 0.99,
];

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct AssetLabel {
    pub owner_id: i32,
}

#[derive(Clone, Debug)]
pub struct WorkerMetric {
    asset_insert_duration: Family<AssetLabel, Histogram>,
    asset_insert_row_change: Family<AssetLabel, Gauge>,
    asset_delete_duration: Family<AssetLabel, Histogram>,
    asset_delete_row_change: Family<AssetLabel, Gauge>,

    blueprint_insert_duration: Family<AssetLabel, Histogram>,
    blueprint_insert_row_change: Family<AssetLabel, Gauge>,
    blueprint_delete_duration: Family<AssetLabel, Histogram>,
    blueprint_delete_row_change: Family<AssetLabel, Gauge>,
}

impl WorkerMetric {
    pub fn new() -> Self {
        Self {
            asset_insert_duration: Family::new_with_constructor(|| {
                Histogram::new(BUCKETS.into_iter())
            }),
            asset_insert_row_change: Family::<AssetLabel, Gauge>::default(),
            asset_delete_duration: Family::new_with_constructor(|| {
                Histogram::new(BUCKETS.into_iter())
            }),
            asset_delete_row_change: Family::<AssetLabel, Gauge>::default(),

            blueprint_insert_duration: Family::new_with_constructor(|| {
                Histogram::new(BUCKETS.into_iter())
            }),
            blueprint_insert_row_change: Family::<AssetLabel, Gauge>::default(),
            blueprint_delete_duration: Family::new_with_constructor(|| {
                Histogram::new(BUCKETS.into_iter())
            }),
            blueprint_delete_row_change: Family::<AssetLabel, Gauge>::default(),
        }
    }

    pub fn asset_insert_duration(
        &self,
        owner_id: i32,
        duration: u128,
    ) {
        self.asset_insert_duration.get_or_create(
            &AssetLabel {
                owner_id,
            }
        ).observe(duration as f64);
    }
    pub fn asset_insert_row_change(
        &self,
        owner_id: i32,
        count:    u64,
    ) {
        self.asset_insert_row_change.get_or_create(
            &AssetLabel {
                owner_id,
            }
        ).set(count as i64);
    }
    pub fn asset_delete_duration(
        &self,
        owner_id: i32,
        duration: u128,
    ) {
        self.asset_delete_duration.get_or_create(
            &AssetLabel {
                owner_id,
            }
        ).observe(duration as f64);
    }
    pub fn asset_delete_row_change(
        &self,
        owner_id: i32,
        count:    u64,
    ) {
        self.asset_delete_row_change.get_or_create(
            &AssetLabel {
                owner_id,
            }
        ).set(count as i64);
    }

    pub fn blueprint_insert_duration(
        &self,
        owner_id: i32,
        duration: u128,
    ) {
        self.blueprint_insert_duration.get_or_create(
            &AssetLabel {
                owner_id,
            }
        ).observe(duration as f64);
    }
    pub fn blueprint_insert_row_change(
        &self,
        owner_id: i32,
        count:    u64,
    ) {
        self.blueprint_insert_row_change.get_or_create(
            &AssetLabel {
                owner_id,
            }
        ).set(count as i64);
    }
    pub fn blueprint_delete_duration(
        &self,
        owner_id: i32,
        duration: u128,
    ) {
        self.blueprint_delete_duration.get_or_create(
            &AssetLabel {
                owner_id,
            }
        ).observe(duration as f64);
    }
    pub fn blueprint_delete_row_change(
        &self,
        owner_id: i32,
        count:    u64,
    ) {
        self.blueprint_delete_row_change.get_or_create(
            &AssetLabel {
                owner_id,
            }
        ).set(count as i64);
    }
}

impl TaskMetric for WorkerMetric {
    fn register(
        &self,
        registry: &mut Registry,
    ) {
        registry.register(
            "asset_insert_duration",
            "Time it took until all asset inserts where done",
            self.asset_insert_duration.clone(),
        );
        registry.register(
            "asset_insert_row_change",
            "Count of how many insert changes were made",
            self.asset_insert_row_change.clone(),
        );
        registry.register(
            "asset_delete_duration",
            "Time it took until the asset cleanup was done",
            self.asset_delete_duration.clone(),
        );
        registry.register(
            "asset_delete_row_change",
            "Count of how many delete changes were made",
            self.asset_delete_row_change.clone(),
        );

        registry.register(
            "blueprint_insert_duration",
            "Time it took until all blueprint inserts where done",
            self.blueprint_insert_duration.clone(),
        );
        registry.register(
            "blueprint_insert_row_change",
            "Count of how many insert changes were made",
            self.blueprint_insert_row_change.clone(),
        );
        registry.register(
            "blueprint_delete_duration",
            "Time it took until the blueprint cleanup was done",
            self.blueprint_delete_duration.clone(),
        );
        registry.register(
            "blueprint_delete_row_change",
            "Count of how many delete changes were made",
            self.blueprint_delete_row_change.clone(),
        );
    }
}
