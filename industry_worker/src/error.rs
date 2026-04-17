use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("error while fetching jobs, error: '{0}'")]
    ListJobs(sqlx::Error),

    #[error("error during transaction, error: '{0}'")]
    TransactionError(sqlx::Error),

    #[error("could not parse additional data")]
    ParseAdditionalData,

    #[error("error while syncing, error: '{0}'")]
    SyncError(sqlx::Error),
    #[error("invalid worker task: '{0}'")]
    InvalidWorkerTask(String),

    #[error(transparent)]
    WorkerLibError(#[from] starfoundry_lib_worker::Error),
    #[error(transparent)]
    EveGatewayError(#[from] starfoundry_lib_eve_gateway::Error),
}
