use chrono::NaiveDateTime;
use prometheus_client::encoding::EncodeLabelValue;
use starfoundry_lib_worker::WorkerTask;

use crate::error::MarketWorkerError;

#[derive(
    Clone, Copy, Debug, Hash, PartialEq, Eq, EncodeLabelValue, sqlx::Type,
)]
#[sqlx(type_name = "WORKER_TASK")]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WorkerMarketTask {
    /// sync the markets with the structure in the industry tool
    Sync,
    /// cleanup old tasks
    Cleanup,

    /// fetches the latest NPC orders
    LatestNpc,
    /// fetches the latest player orders
    LatestPlayer,
    /// fetches the latest region orders
    LatestRegion,

    /// public contracts
    PublicContracts,
    /// public contract items
    PublicContractItems,

    CharacterOrders,
    CorporationOrders,

    /// fetches the latest market prices
    Prices,
}

impl WorkerTask for WorkerMarketTask {
    fn wait_until(&self) -> Option<NaiveDateTime> {
        match self {
            Self::Sync => self.add_minutes(5),
            Self::Cleanup => self.during_downtime(),
            Self::LatestNpc => self.add_minutes(5),
            Self::LatestPlayer => self.add_minutes(5),
            Self::LatestRegion => self.add_minutes(5),
            Self::PublicContracts => self.add_minutes(30),
            Self::PublicContractItems => self.oneshot(),
            Self::CharacterOrders => self.add_minutes(20),
            Self::CorporationOrders => self.add_minutes(20),
            Self::Prices => self.add_minutes(60),
        }
    }
}

impl TryFrom<String> for WorkerMarketTask {
    type Error = MarketWorkerError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_ref() {
            "SYNC" => Ok(Self::Sync),
            "CLEANUP" => Ok(Self::Cleanup),
            "LATEST_NPC" => Ok(Self::LatestNpc),
            "LATEST_PLAYER" => Ok(Self::LatestPlayer),
            "LATEST_REGION" => Ok(Self::LatestRegion),
            "PRICES" => Ok(Self::Prices),
            "PUBLIC_CONTRACTS" => Ok(Self::PublicContracts),
            "PUBLIC_CONTRACT_ITEMS" => Ok(Self::PublicContractItems),
            "CHARACTER_ORDERS" => Ok(Self::CharacterOrders),
            "CORPORATION_ORDERS" => Ok(Self::CorporationOrders),
            _ => Err(MarketWorkerError::InvalidWorkerTask(value)),
        }
    }
}

impl From<WorkerMarketTask> for String {
    fn from(value: WorkerMarketTask) -> String {
        match value {
            WorkerMarketTask::Sync => "SYNC",
            WorkerMarketTask::Cleanup => "CLEANUP",
            WorkerMarketTask::LatestNpc => "LATEST_NPC",
            WorkerMarketTask::LatestPlayer => "LATEST_PLAYER",
            WorkerMarketTask::LatestRegion => "LATEST_REGION",
            WorkerMarketTask::PublicContracts => "PUBLIC_CONTRACTS",
            WorkerMarketTask::PublicContractItems => "PUBLIC_CONTRACT_ITEMS",
            WorkerMarketTask::CharacterOrders => "CHARACTER_ORDERS",
            WorkerMarketTask::CorporationOrders => "CORPORATION_ORDERS",
            WorkerMarketTask::Prices => "PRICES",
        }
        .into()
    }
}
