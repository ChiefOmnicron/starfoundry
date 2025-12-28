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
    #[error("cleaning orders for structure '{1}', error: '{0}'")]
    CleanupOrdersError(sqlx::Error, StructureId),

    #[error("insert orders for region '{1}', error: '{0}'")]
    InsertRegionOrders(sqlx::Error, RegionId),
    #[error("delete orders for region '{1}', error: '{0}'")]
    DeleteRegionOrders(sqlx::Error, RegionId),

    #[error("could not parse additional data")]
    ParseAdditionalData,

    #[error(transparent)]
    WorkerLibError(#[from] starfoundry_lib_worker::Error),
    #[error(transparent)]
    EveGatewayError(#[from] starfoundry_lib_eve_gateway::Error),
}
