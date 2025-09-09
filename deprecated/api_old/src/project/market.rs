mod add;
mod delete;
mod fetch;
mod fetch_prices_gas;
mod fetch_prices;
mod fetch_prices_mineral;
mod last_fetch;
mod update;
mod update_bulk;
mod update_minerals;

pub use self::add::*;
pub use self::delete::*;
pub use self::fetch::*;
pub use self::fetch_prices_gas::*;
pub use self::fetch_prices::*;
pub use self::fetch_prices_mineral::*;
pub use self::last_fetch::*;
pub use self::update::*;
pub use self::update_bulk::*;
pub use self::update_minerals::*;

use sqlx::PgPool;
use starfoundry_lib_eve_api::Credentials;
use starfoundry_lib_projects::{ProjectUuid, ProjectMarketUuid};
use starfoundry_lib_structures::StructureUuid;
use utoipa::IntoParams;
use warp::Filter;
use warp::filters::BoxedFilter;
use warp::reply::Reply;

use crate::{with_identity, with_pool};

pub fn api(
    pool:        PgPool,
    base_path:   BoxedFilter<()>,
    credentials: Credentials
) -> BoxedFilter<(impl Reply,)> {
    let path = base_path
        .clone()
        .and(with_pool(pool.clone()))
        .and(with_identity(pool.clone(), credentials))
        .and(warp::path!("projects" / ProjectUuid / "market" / ..));

    let add = path
        .clone()
        .and(warp::path::end())
        .and(warp::post())
        .and(warp::body::json())
        .and_then(add);

    let delete = path
        .clone()
        .and(warp::path!(ProjectMarketUuid))
        .and(warp::delete())
        .and_then(delete);

    let fetch = path
        .clone()
        .and(warp::path::end())
        .and(warp::get())
        .and_then(fetch);

    let fetch_prices = path
        .clone()
        .and(warp::path!("prices"))
        .and(warp::get())
        .and_then(fetch_prices);

    let fetch_prices_gas = path
        .clone()
        .and(warp::path!("prices" / "gas"))
        .and(warp::get())
        .and_then(fetch_prices_gas);

    let fetch_prices_minerals = path
        .clone()
        .and(warp::path!("prices" / "minerals"))
        .and(warp::get())
        .and_then(fetch_prices_minerals);

    let update = path
        .clone()
        .and(warp::path!(ProjectMarketUuid))
        .and(warp::put())
        .and(warp::body::json())
        .and_then(update);

    let update_bulk = path
        .clone()
        .and(warp::path!("bulk"))
        .and(warp::put())
        .and(warp::body::json())
        .and_then(update_bulk);

    let update_minerals = path
        .clone()
        .and(warp::path!("minerals"))
        .and(warp::put())
        .and(warp::body::json())
        .and_then(update_minerals);

    let last_fetch = base_path
        .and(with_pool(pool.clone()))
        .and(warp::path!("projects" / "market" / StructureUuid / "last-fetch"))
        .and_then(last_fetch);

    add
        .or(delete)
        .or(fetch)
        .or(fetch_prices)
        .or(fetch_prices_gas)
        .or(fetch_prices_minerals)
        .or(update)
        .or(update_bulk)
        .or(update_minerals)
        .or(last_fetch)
        .boxed()
}

#[derive(IntoParams)]
#[into_params(names("projectMarketUuid"))]
pub struct ProjectMarketUuidPath(pub ProjectMarketUuid);
