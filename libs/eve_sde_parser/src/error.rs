use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Holds all errors within the application
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    #[error("the requested file does not exist at the location")]
    FileDoesNotExist(String),
    #[error("there was an error while creating a transaction")]
    TransactionError(sqlx::Error),

    #[error("the file `blueprints.yaml` couldn`t be parsed")]
    ParseBlueprints(serde_yaml::Error),
    #[error("the file `blueprints.yaml` is not at the expected location")]
    CannotOpenBlueprintsFile((std::io::Error, String)),
    #[error("error while deleting the blueprint json database")]
    DeleteBlueprintJson(sqlx::Error),
    #[error("inserting the blueprint data failed")]
    InsertBlueprintJson(sqlx::Error),

    #[error("error while deleting the blueprint json database")]
    DeleteStructureDogma(sqlx::Error),
    #[error("inserting the blueprint data failed")]
    InsertStructureDogma(sqlx::Error),

    #[error("error while deleting the items database")]
    DeleteItems(sqlx::Error),
    #[error("inserting the item data failed")]
    InsertItems(sqlx::Error),
    #[error("inserting the item reprocessing data failed")]
    InsertItemReprocessing(sqlx::Error),

    #[error("the file `group_ids.yaml` couldn`t be parsed")]
    ParseDogmaEffects(serde_yaml::Error),
    #[error("the file `group_ids.yaml` is not at the expected location")]
    CannotOpenDogmaEffectsFile((std::io::Error, String)),

    #[error("the file `group_ids.yaml` couldn`t be parsed")]
    ParseGroupIds(serde_yaml::Error),
    #[error("the file `group_ids.yaml` is not at the expected location")]
    CannotOpenGroupIdsFile((std::io::Error, String)),

    #[error("the file `type_ids.yaml` couldn`t be parsed")]
    ParseTypeIds(serde_yaml::Error),
    #[error("the file `type_ids.yaml` is not at the expected location")]
    CannotOpenTypeIdsFile((std::io::Error, String)),

    #[error("the file `industrymodifiersources` couldn`t be parsed")]
    ParseIndustryModifierSources(serde_json::Error),
    #[error("the file `industrymodifiersources.json` is not at the expected location")]
    CannotOpenIndustryModifierSources((std::io::Error, String)),

    #[error("the file `industrytargetfilters` couldn`t be parsed")]
    ParseIndustryTargetFilters(serde_json::Error),
    #[error("the file `industrytargetfilters.json` is not at the expected location")]
    CannotOpenIndustryTargetFilters((std::io::Error, String)),

    #[error("the file `repackagedvolumes` couldn`t be parsed")]
    ParseRepackagedVolumes(serde_json::Error),
    #[error("the file `repackagedvolumes.json` is not at the expected location")]
    CannotOpenRepackagedVolumes((std::io::Error, String)),

    #[error("the file `typeDogma` couldn`t be parsed")]
    ParseTypeDogma(serde_yaml::Error),
    #[error("the file `typeDogma` is not at the expected location")]
    CannotOpenTypeDogmaFile((std::io::Error, String)),

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
