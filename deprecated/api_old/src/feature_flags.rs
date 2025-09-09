use warp::{Filter, Reply, Rejection};
use warp::filters::BoxedFilter;

const FEATURE_FLAG_JANICE: &'static str = "FEATURE_FLAG_JANICE";

pub fn api(
    base_path: BoxedFilter<()>,
) -> BoxedFilter<(impl Reply,)> {
    let base_path = base_path
        .clone()
        .and(warp::path!("feature-flags" / ..))
        .boxed();

    let list = base_path
        .clone()
        .and(warp::get())
        .and(warp::path::end())
        .and_then(list);

    list
        .boxed()
}

async fn list() -> Result<impl Reply, Rejection> {
    let mut feature_flags: Vec<String> = Vec::new();

    if feature_flag_enabled(FEATURE_FLAG_JANICE) {
        feature_flags.push("JANICE".into());
    }

    Ok(warp::reply::json(&feature_flags))
}

fn feature_flag_enabled(
    flag: &str,
) -> bool {
    match std::env::var(flag) {
        Ok(x)  => {
            if x == "true" || x == "enabled" {
                true
            } else {
                false
            }
        },
        Err(_) => false
    }
}
