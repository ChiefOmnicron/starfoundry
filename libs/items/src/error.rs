use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    #[error("error while fetching items, error: '{0}'")]
    Fetch(sqlx::Error)
}
