use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("error creating transaction, '{0}'")]
    BeginTransaction(sqlx::Error),
    #[error("error committing transaction, '{0}'")]
    CommitTransaction(sqlx::Error),

    #[error("could not parse additional data")]
    ParseAdditionalData,

    #[error("error while syncing, error: '{0}'")]
    SyncError(sqlx::Error),

    #[error("error while inserting assets for '{1}', error: '{0}'")]
    InsertAssetError(sqlx::Error, i32),
    #[error("error while inserting blueprints for '{1}', error: '{0}'")]
    InsertBlueprintsError(sqlx::Error, i32),

    #[error("error while cleaning items for '{1}', error: '{0}'")]
    CleanupItems(sqlx::Error, i32),

    #[error("error while inserting system index, error: '{0}'")]
    InsertSystemIndex(sqlx::Error),
    #[error("error while compressing system index, error: '{0}'")]
    CompressSystemIndex(sqlx::Error),

    #[error("generic sqlx error: '{0}'")]
    GenericSqlxError(sqlx::Error),

    #[error(transparent)]
    WorkerLibError(#[from] starfoundry_lib_worker::Error),
    #[error(transparent)]
    EveGatewayError(#[from] starfoundry_lib_eve_gateway::Error),
}
