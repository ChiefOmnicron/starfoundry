use warp::Filter;
use warp::filters::BoxedFilter;
use warp::reject::Rejection;
use warp::reply::Reply;

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
        .and_then(version_api)
        .boxed();

    version.boxed()
}

#[utoipa::path(
    get,
    operation_id = "version",
    path = "/api/v1/version",
    tag = "version",
    responses(
        (
            body = String,
            content_type = "application/json",
            description = "Gets the current version of the program. Usually the git tag + git version",
            status = OK,
        ),
    ),
)]
async fn version_api(
) -> Result<impl Reply, Rejection> {
    let version = std::env!("GIT_HEAD_SHORT");

    Ok(
        warp::reply::json(&version),
    )
}
