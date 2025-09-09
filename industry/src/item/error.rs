use starfoundry_lib_types::TypeId;
use thiserror::Error;

pub type Result<T, E = ItemError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ItemError {
    #[error("error while fetching item '{1}', error: '{0}'")]
    FetchItem(sqlx::Error, TypeId),
}

