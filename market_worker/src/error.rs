use thiserror::Error;
use starfoundry_lib_types::{RegionId, StructureId};

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("error creating transaction, '{0}'")]
    BeginTransaction(sqlx::Error),
    #[error("error committing transaction, '{0}'")]
    CommitTransaction(sqlx::Error),

    #[error("insert orders for structure '{1}', error: '{0}'")]
    InsertStationOrdersError(sqlx::Error, StructureId),
    #[error("insert order history, error: '{0}'")]
    InsertHistoryOrders(sqlx::Error),
    #[error("cleaning orders for structure '{1}', error: '{0}'")]
    CleanupOrdersError(sqlx::Error, StructureId),

    #[error("insert private orders, error: '{0}'")]
    InsertPrivateOrders(sqlx::Error),
    #[error("insert private order history, error: '{0}'")]
    InsertPrivateHistoryOrders(sqlx::Error),

    #[error("insert orders for region '{1}', error: '{0}'")]
    InsertRegionOrders(sqlx::Error, RegionId),
    #[error("delete orders for region '{1}', error: '{0}'")]
    DeleteRegionOrders(sqlx::Error, RegionId),

    #[error("insert prices, error: '{0}'")]
    InsertMarketPrices(sqlx::Error),

    #[error("could not parse additional data")]
    ParseAdditionalData,

    #[error("insert prices, error: '{0}'")]
    CleanupPublicContracts(sqlx::Error),

    #[error("error while syncing, error: '{0}'")]
    SyncError(sqlx::Error),
    #[error("invalid worker task: '{0}'")]
    InvalidWorkerTask(String),

    #[error(transparent)]
    WorkerLibError(#[from] starfoundry_lib_worker::Error),
    #[error(transparent)]
    EveGatewayError(#[from] starfoundry_lib_eve_gateway::Error),
    #[error(transparent)]
    IndustryError(#[from] starfoundry_lib_industry::Error),
}
