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
    SystemIndexCompress,

    /// Fetches standings for alliance, corporation and character
    AllianceStanding,
    CharacterStanding,
    CorporationStanding,

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
            Self::SystemIndexCompress   => self.during_downtime(),
            Self::AllianceStanding      => self.add_minutes(60),
            Self::CharacterStanding     => self.add_minutes(60),
            Self::CorporationStanding   => self.add_minutes(60),
        }
    }
}

impl TryFrom<String> for WorkerEveGatewayTask {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_ref() {
            "SYNC"                      => Ok(Self::Sync),
            "CHARACTER_ASSETS"          => Ok(Self::CharacterAssets),
            "CORPORATION_ASSETS"        => Ok(Self::CorporationAssets),
            "CHARACTER_BLUEPRINTS"      => Ok(Self::CharacterBlueprints),
            "CORPORATION_BLUEPRINTS"    => Ok(Self::CorporationBlueprints),
            "SYSTEM_INDEX"              => Ok(Self::SystemIndex),
            "SYSTEM_INDEX_COMPRESS"     => Ok(Self::SystemIndexCompress),
            "ALLIANCE_STANDING"         => Ok(Self::AllianceStanding),
            "CHARACTER_STANDING"        => Ok(Self::CharacterStanding),
            "CORPORATION_STANDING"      => Ok(Self::CorporationStanding),
            _                           => Err("Invalid".into()),
        }
    }
}

impl From<WorkerEveGatewayTask> for String {
    fn from(value: WorkerEveGatewayTask) -> Self {
        match value {
            WorkerEveGatewayTask::Sync                  => "SYNC",
            WorkerEveGatewayTask::CharacterAssets       => "CHARACTER_ASSETS",
            WorkerEveGatewayTask::CorporationAssets     => "CORPORATION_ASSETS",
            WorkerEveGatewayTask::CharacterBlueprints   => "CHARACTER_BLUEPRINTS",
            WorkerEveGatewayTask::CorporationBlueprints => "CORPORATION_BLUEPRINTS",
            WorkerEveGatewayTask::SystemIndex           => "SYSTEM_INDEX",
            WorkerEveGatewayTask::SystemIndexCompress   => "SYSTEM_INDEX_COMPRESS",
            WorkerEveGatewayTask::AllianceStanding      => "ALLIANCE_STANDING",
            WorkerEveGatewayTask::CharacterStanding     => "CHARACTER_STANDING",
            WorkerEveGatewayTask::CorporationStanding   => "CORPORATION_STANDING",
        }.into()
    }
}
