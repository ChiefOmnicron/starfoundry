use serde::Serialize;
use sqlx::PgPool;
use starfoundry_lib_types::{TypeId, CategoryId, GroupId};
use warp::{Filter, Reply, Rejection};
use warp::filters::BoxedFilter;

use crate::with_pool;

pub mod error;
pub use self::error::*;
use utoipa::ToSchema;

mod all;
mod blueprint_originals;
mod blueprints;
mod buildable;
mod parse;
mod resolve_bulk_names;
mod resolve_id;

pub mod service {
    pub use super::all::*;
    pub use super::blueprint_originals::*;
    pub use super::blueprints::*;
    pub use super::buildable::*;
    pub use super::parse::*;
    pub use super::resolve_bulk_names::*;
    pub use super::resolve_id::*;
}

pub fn api(
    pool:             PgPool,
    base_path:        BoxedFilter<()>,
) -> BoxedFilter<(impl Reply,)> {
    let base_path = base_path
        .clone()
        .and(warp::path!("items" / ..))
        .and(with_pool(pool.clone()))
        .boxed();

    // TODO: doc
    let all = base_path
        .clone()
        .and(warp::path::end())
        .and(warp::get())
        .and_then(all)
        .boxed();

    // TODO: doc
    let buildable = base_path
        .clone()
        .and(warp::path!("buildable"))
        .and(warp::get())
        .and_then(buildable)
        .boxed();

    // TODO: doc
    let blueprints = base_path
        .clone()
        .and(warp::path!("blueprints"))
        .and(warp::get())
        .and_then(blueprints)
        .boxed();

    // TODO: doc
    let resolve_bulk_names = base_path
        .clone()
        .and(warp::path!("resolve" / "names"))
        .and(warp::post())
        .and(warp::body::json())
        .and_then(resolve_bulk_names)
        .boxed();

    // TODO: doc
    let resolve_id = base_path
        .clone()
        .and(warp::path!("resolve" / "ids" / TypeId))
        .and(warp::get())
        .and_then(resolve_id)
        .boxed();

    // TODO: doc
    let resolve_id_bulk = base_path
        .clone()
        .and(warp::path!("resolve" / "ids"))
        .and(warp::post())
        .and(warp::body::json())
        .and_then(resolve_id_bulk)
        .boxed();

    // TODO: doc
    let blueprint_originals = base_path
        .clone()
        .and(warp::path!("blueprints" / "originals"))
        .and(warp::get())
        .and_then(blueprint_originals)
        .boxed();

    // TODO: doc
    let parse = base_path
        .clone()
        .and(warp::path!("parse"))
        .and(warp::post())
        .and(warp::body::json())
        .and_then(parse)
        .boxed();

    all
        .or(blueprint_originals)
        .or(blueprints)
        .or(buildable)
        .or(parse)
        .or(resolve_bulk_names)
        .or(resolve_id)
        .or(resolve_id_bulk)
        .boxed()
}

async fn all(
    pool: PgPool,
) -> Result<impl Reply, Rejection> {
    self::service::all(&pool)
        .await
        .map(|x| warp::reply::json(&x))
        .map_err(Into::into)
}

async fn buildable(
    pool: PgPool,
) -> Result<impl Reply, Rejection> {
    self::service::buildable(&pool)
        .await
        .map(|x| warp::reply::json(&x))
        .map_err(Into::into)
}

async fn blueprints(
    pool: PgPool,
) -> Result<impl Reply, Rejection> {
    self::service::blueprints(&pool)
        .await
        .map(|x| warp::reply::json(&x))
        .map_err(Into::into)
}

async fn blueprint_originals(
    pool:    PgPool,
) -> Result<impl Reply, Rejection> {
    self::service::blueprint_originals(&pool)
        .await
        .map(|x| warp::reply::json(&x))
        .map_err(Into::into)
}

async fn parse(
    pool:    PgPool,
    content: String,
) -> Result<impl Reply, Rejection> {
    self::service::parse_items(&pool, content)
        .await
        .map(|x| warp::reply::json(&x))
        .map_err(Into::into)
}

async fn resolve_id(
    pool:    PgPool,
    type_id: TypeId,
) -> Result<impl Reply, Rejection> {
    self::service::resolve_id(&pool, type_id)
        .await
        .map(|x| warp::reply::json(&x))
        .map_err(Into::into)
}

async fn resolve_id_bulk(
    pool:     PgPool,
    type_ids: Vec<TypeId>,
) -> Result<impl Reply, Rejection> {
    let mut resolved = Vec::new();

    for type_id in type_ids {
        resolved.push(self::service::resolve_id(&pool, type_id).await?);
    }

    Ok(warp::reply::json(&resolved))
}

async fn resolve_bulk_names(
    pool:  PgPool,
    names: Vec<String>,
) -> Result<impl Reply, Rejection> {
    self::service::resolve_bulk_names(&pool, names)
        .await
        .map(|x| warp::reply::json(&x))
        .map_err(Into::into)
}

#[derive(Debug, Serialize, ToSchema)]
#[cfg_attr(test, derive(serde::Deserialize))]
pub struct Item {
    pub type_id:     TypeId,
    pub category_id: CategoryId,
    pub group_id:    GroupId,
    pub volume:      f32,
    pub name:        String,
    pub base_price:  Option<f32>,
}

impl Item {
    // TODO: refactor
    pub async fn new(
        pool:    &PgPool,
        type_id: TypeId,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let item = sqlx::query_as!(
            Item,
            r#"
                SELECT
                    type_id,
                    category_id,
                    group_id,
                    volume,
                    name,
                    base_price
                FROM item
                -- Exclude some of the categories that we won´t need
                WHERE type_id = $1
                ORDER BY name
            "#,
                *type_id,
            )
            .fetch_one(pool)
            .await?;
        Ok(item)
    }
}
