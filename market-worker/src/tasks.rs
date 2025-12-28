use chrono::NaiveDateTime;
use prometheus_client::encoding::EncodeLabelValue;
use starfoundry_lib_worker::WorkerTask;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, EncodeLabelValue, sqlx::Type)]
#[sqlx(type_name = "WORKER_TASK")]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WorkerMarketTask {
    /// sync the markets with the structure in the industry tool
    Sync,

    /// fetches the latest NPC orders
    LatestNpc,
    /// fetches the latest player orders
    LatestPlayer,
    /// fetches the latest region orders
    LatestRegion,
    /// fetches the latest market prices
    Prices,
}

impl WorkerTask for WorkerMarketTask {
    fn wait_until(
        &self,
    ) -> NaiveDateTime {
        match self {
            Self::Sync            => self.add_minutes(5),
            Self::LatestNpc       => self.add_minutes(5),
            Self::LatestPlayer    => self.add_minutes(5),
            Self::LatestRegion    => self.add_minutes(5),
            Self::Prices          => self.add_minutes(60),
        }
    }
}

impl TryFrom<String> for WorkerMarketTask {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_ref() {
            "SYNC"          => Ok(Self::Sync),
            "LATEST_NPC"    => Ok(Self::LatestNpc),
            "LATEST_PLAYER" => Ok(Self::LatestPlayer),
            "LATEST_REGION" => Ok(Self::LatestRegion),
            "PRICES"        => Ok(Self::Prices),
            _               => Err("Invalid".into()),
        }
    }
}

impl Into<String> for WorkerMarketTask {
    fn into(self) -> String {
        match self {
            Self::Sync          => "SYNC",
            Self::LatestNpc     => "LATEST_NPC",
            Self::LatestPlayer  => "LATEST_PLAYER",
            Self::LatestRegion  => "LATEST_REGION",
            Self::Prices        => "PRICES",
        }.into()
    }
}
