use sqlx::PgPool;
use starfoundry_libs_eve_api::CredentialCache;
use starfoundry_libs_types::CharacterId;
use std::sync::{Arc, Mutex};
use warp::{Filter, Reply, Rejection};
use warp::filters::BoxedFilter;
use warp::http::StatusCode;

use crate::{Identity, with_identity, with_pool};

pub mod character;
pub use self::character::*;
pub mod error;
pub use self::error::*;

mod alts;
mod corporations;
mod info;
mod remove;
mod save;

pub mod service {
    pub use super::alts::*;
    pub use super::corporations::*;
    pub use super::info::*;
    pub use super::remove::*;
    pub use super::save::*;
}

pub fn api(
    pool:              PgPool,
    base_path:         BoxedFilter<()>,
    credential_cache:  Arc<Mutex<CredentialCache>>,
) -> BoxedFilter<(impl Reply,)> {
    let base_path = base_path
        .clone()
        .and(warp::path!("characters" / ..))
        .and(with_identity(pool.clone(), credential_cache.clone()))
        .and(with_pool(pool.clone()));

    // TODO: doc
    let info = base_path
        .clone()
        .and(warp::path!(CharacterId))
        .and(warp::get())
        .and_then(info)
        .boxed();

    // TODO: doc
    let delete = base_path
        .clone()
        .and(warp::path!(CharacterId))
        .and(warp::delete())
        .and_then(delete)
        .boxed();

    // TODO: doc
    let alts = base_path
        .clone()
        .and(warp::path!("alts"))
        .and(warp::get())
        .and_then(alts)
        .boxed();

    // TODO: doc
    let corporations = base_path
        .clone()
        .and(warp::path!("corporations"))
        .and(warp::get())
        .and_then(corporations)
        .boxed();

    info
        .or(delete)
        .or(alts)
        .or(corporations)
        .boxed()
}

/// Gets infos about the given [CharacterId]
/// 
/// # Params
/// 
/// * `service` -> [CharacterService]
/// 
/// # Errors
/// 
/// If there are problems with the EVE-API Endpoint
/// 
/// # Returns
/// 
/// General information about the character
/// 
async fn info(
    identity:     Identity,
    pool:         PgPool,
    character_id: CharacterId,
) -> Result<impl Reply, Rejection> {
    let client = identity.api_client().await?;

    self::service::info(&pool, &client, character_id)
        .await
        .map(|x| warp::reply::json(&x))
        .map_err(Into::into)
}


/// Gets a list of all alts from the main character.
/// 
/// # Params
/// 
/// * `service` -> [CharacterService]
/// * `user`    -> Instance of an authenticated user
/// 
/// # Errors
/// 
/// If the database access fails.
/// 
/// # Returns
/// 
/// List of all alts from the logged in main character.
/// 
async fn alts(
    identity: Identity,
    pool:     PgPool,
) -> Result<impl Reply, Rejection> {
    let character_id = identity.character_id();
    let client = identity.api_client().await?;

    self::service::alts(&pool, &client, character_id)
        .await
        .map(|x| warp::reply::with_status(
            warp::reply::json(&x),
            StatusCode::OK
        ))
        .map_err(Into::into)
}

async fn corporations(
    identity: Identity,
    pool:     PgPool,
) -> Result<impl Reply, Rejection> {
    let character_id = identity.character_id();

    self::service::corporations(&pool, character_id)
        .await
        .map(|x| warp::reply::with_status(
            warp::reply::json(&x),
            StatusCode::OK
        ))
        .map_err(Into::into)
}

/// Removes the given user.
/// If the user does not exist, nothing will happen.
/// 
/// # Params
/// 
/// `service` -> [CharacterService]
/// `cid`     -> Id of the character that should be removed
/// 
/// # Errors
/// 
/// If the database access fails.
/// 
/// # Returns
/// 
/// Status code 204 - No content
/// 
async fn delete(
    _:            Identity,
    pool:         PgPool,
    character_id: CharacterId,
) -> Result<impl Reply, Rejection> {
    self::service::remove(&pool, character_id).await?;

    Ok(warp::reply::with_status(
        warp::reply::json(&()),
        StatusCode::NO_CONTENT,
    ))
}
