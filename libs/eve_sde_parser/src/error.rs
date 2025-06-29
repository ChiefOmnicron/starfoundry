use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Holds all errors within the application
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    #[error("the requested file does not exist at the location, '{0}'")]
    FileDoesNotExist(String),
    #[error("there was an error while creating a transaction, '{0}'")]
    TransactionError(sqlx::Error),

    #[error("the file `blueprints.yaml` couldn`t be parsed, '{0}'")]
    ParseBlueprints(serde_yaml::Error),
    #[error("the file `blueprints.yaml` is not at the expected location, file '{1}', error '{0}'")]
    CannotOpenBlueprintsFile(std::io::Error, String),
    #[error("error while deleting the blueprint json database, '{0}'")]
    DeleteBlueprintJson(sqlx::Error),
    #[error("inserting the blueprint data failed, '{0}'")]
    InsertBlueprintJson(sqlx::Error),

    #[error("error while deleting the blueprint json database, '{0}'")]
    DeleteStructureDogma(sqlx::Error),
    #[error("inserting the blueprint data failed, '{0}'")]
    InsertStructureDogma(sqlx::Error),

    #[error("error while deleting the items database, '{0}'")]
    DeleteItems(sqlx::Error),
    #[error("inserting the item data failed, '{0}'")]
    InsertItems(sqlx::Error),
    #[error("inserting the item reprocessing data failed, '{0}'")]
    InsertItemReprocessing(sqlx::Error),

    #[error("the file `group_ids.yaml` couldn`t be parsed, '{0}'")]
    ParseDogmaEffects(serde_yaml::Error),
    #[error("the file `group_ids.yaml` is not at the expected location, '{0}'")]
    CannotOpenDogmaEffectsFile(std::io::Error, String),

    #[error("the file `group_ids.yaml` couldn`t be parsed, '{0}'")]
    ParseGroupIds(serde_yaml::Error),
    #[error("the file `group_ids.yaml` is not at the expected location, '{0}'")]
    CannotOpenGroupIdsFile(std::io::Error, String),

    #[error("the file `type_ids.yaml` couldn`t be parsed, '{0}'")]
    ParseTypeIds(serde_yaml::Error),
    #[error("the file `type_ids.yaml` is not at the expected location, '{0}'")]
    CannotOpenTypeIdsFile(std::io::Error, String),

    #[error("the file `industrymodifiersources` couldn`t be parsed, '{0}'")]
    ParseIndustryModifierSources(serde_json::Error),
    #[error("the file `industrymodifiersources.json` is not at the expected location, '{0}'")]
    CannotOpenIndustryModifierSources(std::io::Error, String),

    #[error("the file `industrytargetfilters` couldn`t be parsed, '{0}'")]
    ParseIndustryTargetFilters(serde_json::Error),
    #[error("the file `industrytargetfilters.json` is not at the expected location, '{0}'")]
    CannotOpenIndustryTargetFilters(std::io::Error, String),

    #[error("the file `repackagedvolumes` couldn`t be parsed, '{0}'")]
    ParseRepackagedVolumes(serde_json::Error),
    #[error("the file `repackagedvolumes.json` is not at the expected location, '{0}'")]
    CannotOpenRepackagedVolumes(std::io::Error, String),

    #[error("the file `typeDogma` couldn`t be parsed, '{0}'")]
    ParseTypeDogma(serde_yaml::Error),
    #[error("the file `typeDogma` is not at the expected location, '{0}'")]
    CannotOpenTypeDogmaFile(std::io::Error, String),

    #[error("error while writing file to path '{1}', error: {0}")]
    FileWriteError(std::io::Error, String),

    #[error("generic io error, '{0}'")]
    IoError(std::io::Error),

    #[error("reqwest error, error: {0}")]
    ReqwestError(reqwest::Error),

    #[error("error while unzipping, error: {0}")]
    UnzipError(std::io::Error),
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::ReqwestError(value)
    }
}
