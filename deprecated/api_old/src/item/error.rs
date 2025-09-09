use thiserror::Error;

#[derive(Error, Debug)]
pub enum ItemError {
    #[error("fetching all items {0}")]
    FetchAll(sqlx::Error),
    #[error("fetching blueprint originals")]
    FetchBlueprintOriginals(sqlx::Error),
    #[error("fetching builable items")]
    FetchBuildable(sqlx::Error),
    #[error("resolving item id")]
    ResolveId(sqlx::Error),
    #[error("resolving item names")]
    BulkResolveName(sqlx::Error),

    #[error("Error loading item cache {0}")]
    BuildupItemCache(starfoundry_lib_items::Error),
}

impl warp::reject::Reject for ItemError { }
