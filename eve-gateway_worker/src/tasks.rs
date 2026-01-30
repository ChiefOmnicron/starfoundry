use chrono::NaiveDateTime;
use prometheus_client::encoding::EncodeLabelValue;
use starfoundry_lib_worker::WorkerTask;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, EncodeLabelValue, sqlx::Type)]
#[sqlx(type_name = "WORKER_TASK")]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WorkerEveGatewayTask {
    /// makes sure that all tasks are set
    Sync,

    /// Loads all character assets
    CharacterAssets,
    /// Loads all corporation assets
    CorporationAssets,

    /// Loads all character blueprints
    CharacterBlueprints,
    /// Loads all corporation blueprints
    CorporationBlueprints,

    /// Fetches the system index for all systems
    SystemIndex,
    //CompressSystemIndex,

    // Skills
    // Industry Jobs Character
    // Industry Jobs Corporation
}

impl WorkerTask for WorkerEveGatewayTask {
    fn wait_until(
        &self,
    ) -> Option<NaiveDateTime> {
        match self {
            Self::Sync                  => self.add_minutes(5),
            Self::CharacterAssets       => self.add_minutes(60),
            Self::CorporationAssets     => self.add_minutes(60),
            Self::CharacterBlueprints   => self.add_minutes(60),
            Self::CorporationBlueprints => self.add_minutes(60),
            Self::SystemIndex           => self.add_minutes(60),
        }
    }
}

impl TryFrom<String> for WorkerEveGatewayTask {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_ref() {
            "SYNC"                   => Ok(Self::Sync),
            "CHARACTER_ASSETS"       => Ok(Self::CharacterAssets),
            "CORPORATION_ASSETS"     => Ok(Self::CorporationAssets),
            "CHARACTER_BLUEPRINTS"   => Ok(Self::CharacterBlueprints),
            "CORPORATION_BLUEPRINTS" => Ok(Self::CorporationBlueprints),
            "SYSTEM_INDEX"           => Ok(Self::SystemIndex),
            _                        => Err("Invalid".into()),
        }
    }
}

impl Into<String> for WorkerEveGatewayTask {
    fn into(self) -> String {
        match self {
            Self::Sync                  => "SYNC",
            Self::CharacterAssets       => "CHARACTER_ASSETS",
            Self::CorporationAssets     => "CORPORATION_ASSETS",
            Self::CharacterBlueprints   => "CHARACTER_BLUEPRINTS",
            Self::CorporationBlueprints => "CORPORATION_BLUEPRINTS",
            Self::SystemIndex           => "SYSTEM_INDEX",
        }.into()
    }
}
