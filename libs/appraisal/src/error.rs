use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum Error {
    /// Thrown when a ENV is missing, contains the name of the missing ENV
    #[error("missing environment variables {0}")]
    MissingEnv(String),

    /// Error when constructing a rest client fails
    #[error("a reqwest client couldn´t be created, {0}")]
    CouldNotConstructClient(reqwest::Error),

    /// Error during request
    #[error("api request error, {0}")]
    RequestError(reqwest::Error),

    #[error("error fetching market prices for internal appraisal tool, '{0}'")]
    FetchInternalMarketPrices(sqlx::Error),

    #[error("error while loading item cache, error: '{0}'")]
    LoadItemCache(starfoundry_lib_items::Error),
    #[error("general database error, error: '{0}'")]
    DatabaseError(sqlx::Error),

    #[error("no solution for compression")]
    NoSolution,
}
