use starfoundry_libs_types::StationId;
use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum Error {
    // npc and player market orders
    #[error("error while fetching market stations, error: {0}")]
    FetchMarketStations(sqlx::Error),
    #[error("error while inserting market station '{1}', error: {0}")]
    InsertMarketOrders(sqlx::Error, StationId),
    #[error("error deleting current sell orders for station '{1}', error: {0}")]
    DeleteSellOrders(sqlx::Error, StationId),

    // market prices
    #[error("error while inserting market prices, error: {0}")]
    InsertMarketPrices(sqlx::Error),

    // industry index
    #[error("error while inserting industry index, error {0}")]
    InsertIndustryIndex(sqlx::Error),
    #[error("error while cleaning up industry index, error {0}")]
    CleanupIndustryIndex(sqlx::Error),

    // utils
    #[error("error while fetching corporation ids, error: {0}")]
    FetchCorporationIds(sqlx::Error),
    #[error("error while fetching character ids, error: {0}")]
    FetchCharacterIds(sqlx::Error),

    // general errors
    #[error("error while starting transaction, error: {0}")]
    BeginTransaction(sqlx::Error),
    #[error("error while commiting transaction, error: {0}")]
    CommitTransaction(sqlx::Error),
    #[error("error while requesting from the EVE-API, error {0}")]
    ConnectError(starfoundry_libs_eve_api::Error),
}
