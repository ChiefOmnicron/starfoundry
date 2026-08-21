use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("error during transaction, '{0}'")]
    Transaction(sqlx::Error),

    #[error("could not parse additional data")]
    ParseAdditionalData,

    #[error("error while syncing, error: '{0}'")]
    Sync(sqlx::Error),

    #[error("error while inserting assets for '{1}', error: '{0}'")]
    InsertAsset(sqlx::Error, i32),
    #[error("error while inserting blueprints for '{1}', error: '{0}'")]
    InsertBlueprints(sqlx::Error, i32),

    #[error("error while cleaning standings, error: '{0}'")]
    CleanupStandings(sqlx::Error),
    #[error("error while inserting standings, error: '{0}'")]
    InsertStandings(sqlx::Error),

    #[error("error while cleaning items for '{1}', error: '{0}'")]
    CleanupItems(sqlx::Error, i32),

    #[error("error while inserting system index, error: '{0}'")]
    InsertSystemIndex(sqlx::Error),
    #[error("error while compressing system index, error: '{0}'")]
    CompressSystemIndex(sqlx::Error),

    #[error("generic sqlx error: '{0}'")]
    GenericSqlx(sqlx::Error),

    #[error(transparent)]
    WorkerLib(#[from] starfoundry_lib_worker::Error),
    #[error(transparent)]
    EveGateway(#[from] starfoundry_lib_eve_gateway::Error),
}
