use warp::Filter;
use warp::filters::BoxedFilter;
use warp::reject::Rejection;
use warp::reply::Reply;
use serde::Serialize;
use utoipa::ToSchema;

pub fn api(
    base_path: BoxedFilter<()>,
) -> BoxedFilter<(impl Reply,)> {
    let base_path = base_path
        .clone()
        .and(warp::path!("version" / ..))
        .boxed();

    let version = base_path
        .clone()
        .and(warp::path::end())
        .and(warp::get())
        .and_then(version)
        .boxed();

    version.boxed()
}

/// /version
/// 
/// Returns the git reference of the api server
/// 
#[utoipa::path(
    get,
    operation_id = "version",
    path = "/version",
    tag = "version",
    responses(
        (
            body = Version,
            content_type = "application/json",
            description = "Gets the current version of the program. Usually the git tag + git version",
            status = OK,
        ),
    ),
)]
async fn version(
) -> Result<impl Reply, Rejection> {
    let git_tag = std::env!("GIT_HEAD_SHORT").to_string();
    let version = std::env!("CARGO_PKG_VERSION").to_string();

    Ok(
        warp::reply::json(&Version {
            git_tag,
            version,
        }),
    )
}

#[derive(Debug, Serialize, ToSchema)]
pub struct Version {
    pub git_tag: String,
    pub version: String,
}
