use blueprint::BlueprintStockUuid;
use sqlx::PgPool;
use starfoundry_libs_eve_api::Credentials;
use warp::{Filter, Reply};
use warp::filters::BoxedFilter;

use crate::{with_identity, with_pool};

pub mod blueprint;

pub mod service {
    pub use super::blueprint;
}

/// Filters that build up the api for this part of the application
pub fn api(
    pool:        PgPool,
    base_path:   BoxedFilter<()>,
    credentials: Credentials,
) -> BoxedFilter<(impl Reply,)> {
    let base_path = base_path
        .clone()
        .and(warp::path!("stocks" / ..))
        .boxed();

    let stock_blueprint_list = base_path
        .clone()
        .and(with_pool(pool.clone()))
        .and(with_identity(pool.clone(), credentials.clone()))
        .and(warp::get())
        .and(warp::path!("blueprints"))
        .and_then(service::blueprint::list_api)
        .boxed();

    let stock_blueprint_fetch = base_path
        .clone()
        .and(with_pool(pool.clone()))
        .and(with_identity(pool.clone(), credentials.clone()))
        .and(warp::get())
        .and(warp::path!("blueprints" / BlueprintStockUuid))
        .and_then(service::blueprint::fetch_api)
        .boxed();

    let stock_blueprint_create = base_path
        .clone()
        .and(with_pool(pool.clone()))
        .and(with_identity(pool.clone(), credentials.clone()))
        .and(warp::post())
        .and(warp::path!("blueprints"))
        .and(warp::body::json())
        .and_then(service::blueprint::create_api)
        .boxed();

    let stock_blueprint_update = base_path
        .clone()
        .and(with_pool(pool.clone()))
        .and(with_identity(pool.clone(), credentials.clone()))
        .and(warp::put())
        .and(warp::path!("blueprints" / BlueprintStockUuid))
        .and(warp::body::json())
        .and_then(service::blueprint::update_api)
        .boxed();

    let stock_blueprint_delete = base_path
        .clone()
        .and(with_pool(pool.clone()))
        .and(with_identity(pool.clone(), credentials.clone()))
        .and(warp::delete())
        .and(warp::path!("blueprints" / BlueprintStockUuid))
        .and_then(service::blueprint::delete_api)
        .boxed();

    let stock_blueprint_threshold_add = base_path
        .clone()
        .and(with_pool(pool.clone()))
        .and(with_identity(pool.clone(), credentials.clone()))
        .and(warp::post())
        .and(warp::path!("blueprints" / BlueprintStockUuid / "thresholds"))
        .and(warp::body::json())
        .and_then(service::blueprint::add_threshold_api)
        .boxed();

    let stock_blueprint_thresholds_delete = base_path
        .clone()
        .and(with_pool(pool.clone()))
        .and(with_identity(pool.clone(), credentials.clone()))
        .and(warp::delete())
        .and(warp::path!("blueprints" / BlueprintStockUuid / "thresholds" / BlueprintStockUuid))
        .and_then(service::blueprint::delete_threshold_api)
        .boxed();

    let stock_blueprint_thresholds_fetch = base_path
        .clone()
        .and(with_pool(pool.clone()))
        .and(with_identity(pool.clone(), credentials.clone()))
        .and(warp::get())
        .and(warp::path!("blueprints" / BlueprintStockUuid / "thresholds"))
        .and_then(service::blueprint::fetch_thresholds_api)
        .boxed();

    let stock_blueprint_thresholds_update = base_path
        .clone()
        .and(with_pool(pool.clone()))
        .and(with_identity(pool.clone(), credentials.clone()))
        .and(warp::put())
        .and(warp::path!("blueprints" / BlueprintStockUuid / "thresholds"))
        .and(warp::body::json())
        .and_then(service::blueprint::update_threshold_api)
        .boxed();

    stock_blueprint_list
        .or(stock_blueprint_fetch)
        .or(stock_blueprint_create)
        .or(stock_blueprint_update)
        .or(stock_blueprint_delete)
        .or(stock_blueprint_threshold_add)
        .or(stock_blueprint_thresholds_delete)
        .or(stock_blueprint_thresholds_fetch)
        .or(stock_blueprint_thresholds_update)
        .boxed()
}
