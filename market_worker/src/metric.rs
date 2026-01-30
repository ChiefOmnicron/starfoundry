use prometheus_client::encoding::EncodeLabelSet;
use prometheus_client::metrics::family::Family;
use prometheus_client::metrics::gauge::Gauge;
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

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct ContractLabel {
    pub typ: String,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct PrivateOrderLabel {
    pub issuer_id: i32,
}

#[derive(Clone, Debug)]
pub struct WorkerMetric {
    market_station_changed_rows:    Family<MarketOrderLabel, Gauge>,
    market_station_deleted_rows:    Family<MarketOrderLabel, Gauge>,
    market_station_update_duration: Family<MarketOrderLabel, Histogram>,
    market_station_delete_duration: Family<MarketOrderLabel, Histogram>,

    contract_added_count:           Family<ContractLabel, Gauge>,
    contract_added_duration:        Family<ContractLabel, Histogram>,
    contract_expired_count:         Family<ContractLabel, Gauge>,
    contract_expired_duration:      Family<ContractLabel, Histogram>,

    private_orders_added_count:     Family<PrivateOrderLabel, Gauge>,
    private_orders_added_duration:  Family<PrivateOrderLabel, Histogram>,
}

impl WorkerMetric {
    pub fn new() -> Self {
        Self {
            market_station_changed_rows: Family::<MarketOrderLabel, Gauge>::default(),
            market_station_deleted_rows: Family::<MarketOrderLabel, Gauge>::default(),
            market_station_update_duration: Family::new_with_constructor(|| {
                Histogram::new(BUCKETS.into_iter())
            }),
            market_station_delete_duration: Family::new_with_constructor(|| {
                Histogram::new(BUCKETS.into_iter())
            }),

            contract_added_count:    Family::<ContractLabel, Gauge>::default(),
            contract_added_duration: Family::new_with_constructor(|| {
                Histogram::new(BUCKETS.into_iter())
            }),

            contract_expired_count:    Family::<ContractLabel, Gauge>::default(),
            contract_expired_duration: Family::new_with_constructor(|| {
                Histogram::new(BUCKETS.into_iter())
            }),

            private_orders_added_count:    Family::<PrivateOrderLabel, Gauge>::default(),
            private_orders_added_duration: Family::new_with_constructor(|| {
                Histogram::new(BUCKETS.into_iter())
            }),
        }
    }

    pub fn increase_market_order_rows_changed(
        &self,
        station_id: StructureId,
        amount:     u64,
    ) {
        self.market_station_changed_rows
            .get_or_create(
                &MarketOrderLabel {
                    structure_id: *station_id,
                }
            )
            .set(amount as i64);
    }

    pub fn increase_market_order_rows_deleted(
        &self,
        station_id: StructureId,
        amount:     u64,
    ) {
        self.market_station_deleted_rows
            .get_or_create(
                &MarketOrderLabel {
                    structure_id: *station_id,
                }
            )
            .set(amount as i64);
    }

    pub fn add_market_order_latest_update_duration(
        &self,
        station_id: StructureId,
        duration:   u128,
    ) {
        self.market_station_update_duration
            .get_or_create(
                &MarketOrderLabel {
                    structure_id: *station_id,
                }
            )
            .observe(duration as f64);
    }

    pub fn add_market_order_latest_delete_duration(
        &self,
        station_id: StructureId,
        duration:   u128,
    ) {
        self.market_station_delete_duration
            .get_or_create(
                &&MarketOrderLabel {
                    structure_id: *station_id,
                }
            )
            .observe(duration as f64);
    }

    pub fn increase_added_contract_count(
        &self,
        amount: u64,
    ) {
        self.contract_added_count
            .get_or_create(&ContractLabel {
                typ: "public".into()
            })
            .set(amount as i64);
    }

    pub fn increase_expired_contract_count(
        &self,
        amount: u64,
    ) {
        self.contract_expired_count
            .get_or_create(&ContractLabel {
                typ: "public".into()
            })
            .set(amount as i64);
    }

    pub fn added_contract_duration(
        &self,
        duration: u128,
    ) {
        self.contract_added_duration
            .get_or_create(&ContractLabel {
                typ: "public".into()
            })
            .observe(duration as f64);
    }

    pub fn expired_contract_duration(
        &self,
        duration: u128,
    ) {
        self.contract_expired_duration
            .get_or_create(&ContractLabel {
                typ: "public".into()
            })
            .observe(duration as f64);
    }

    pub fn increase_private_orders_added_count(
        &self,
        issuer_id: i32,
        amount:    u64,
    ) {
        self.private_orders_added_count
            .get_or_create(
                &PrivateOrderLabel {
                    issuer_id,
                }
            )
            .set(amount as i64);
    }

    pub fn add_private_orders_added_duration(
        &self,
        issuer_id: i32,
        duration:  u128,
    ) {
        self.private_orders_added_duration
            .get_or_create(
                &PrivateOrderLabel {
                    issuer_id,
                }
            )
            .observe(duration as f64);
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
            self.market_station_changed_rows.clone(),
        );

        registry.register(
            "market_station_rows_deleted",
            "Number of rows deleted per station",
            self.market_station_deleted_rows.clone(),
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

        // contract
        registry.register(
            "contract_expired_count",
            "Number of contracts that expired",
            self.contract_expired_count.clone(),
        );
        registry.register(
            "contract_expired_duration",
            "Time it took to update expired entries",
            self.contract_expired_duration.clone(),
        );

        // private orders
        registry.register(
            "private_orders_added_count",
            "Number of orders that were added",
            self.private_orders_added_count.clone(),
        );
        registry.register(
            "private_orders_added_duration",
            "Time it took to add the entries",
            self.private_orders_added_duration.clone(),
        );
    }
}
