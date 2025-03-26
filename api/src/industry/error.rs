use starfoundry_libs_types::SystemId;
use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum IndustryError {
    #[error("error fetching system index for system '{1}', error: '{0}'")]
    FetchIndustryIndex(sqlx::Error, SystemId),
}
