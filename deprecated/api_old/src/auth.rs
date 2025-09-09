mod callback;
mod claim;
mod intention;
mod login_alt;
mod login_corporation;
mod login;
mod scopes;

#[cfg(feature = "test")]
mod oracle;

mod error;
mod identity;

pub use self::claim::*;
pub use self::error::*;
pub use self::identity::*;
pub use self::scopes::*;

use chrono::{Utc, Months};
use sqlx::PgPool;
use starfoundry_lib_eve_api::CredentialCache;
use starfoundry_lib_types::CharacterId;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use utoipa::IntoParams;
use uuid::Uuid;
use warp::{Filter, Reply, Rejection};
use warp::filters::BoxedFilter;
use warp::http::header::{LOCATION, SET_COOKIE};
use warp::http::StatusCode;

use crate::{with_pool, with_credential_cache};

/// Url to redirect after login
const REDIRECT: &str = "REDIRECT";

pub fn api(
    pool:             PgPool,
    base_path:        BoxedFilter<()>,
    credential_cache: Arc<Mutex<CredentialCache>>,
) -> BoxedFilter<(impl Reply,)> {
    let base_path = base_path
        .clone()
        .and(warp::path!("auth" / ..))
        .boxed();

    // TODO: doc
    let callback = base_path
        .clone()
        .and(warp::path!("callback"))
        .and(warp::get())
        .and(warp::query::<HashMap<String, String>>())
        .and(with_credential_cache(credential_cache.clone()))
        .and(with_pool(pool.clone()))
        .and_then(callback)
        .boxed();

    // TODO: doc
    let login = base_path
        .clone()
        .and(warp::path!("login"))
        .and(warp::get())
        .and(with_pool(pool.clone()))
        .and_then(login)
        .boxed();

    // TODO: doc
    let login_alt = base_path
        .clone()
        .and(warp::path!("login" / "alt"))
        .and(warp::get())
        .and(with_pool(pool.clone()))
        .and(with_identity(pool.clone(), credential_cache.clone()))
        .and_then(login_alt)
        .boxed();

    // TODO: doc
    let login_corporation = base_path
        .clone()
        .and(warp::path!("login" / "corporation"))
        .and(warp::get())
        .and(with_pool(pool.clone()))
        .and(with_identity(pool.clone(), credential_cache.clone()))
        .and_then(login_corporation)
        .boxed();

    // TODO: doc
    let whoami = base_path
        .clone()
        .and(warp::path!("whoami"))
        .and(warp::get())
        .and(with_pool(pool.clone()))
        .and(with_identity(pool.clone(), credential_cache.clone()))
        .and_then(whoami)
        .boxed();

    // TODO: doc
    #[cfg(feature = "test")]
    let oracle = base_path
        .clone()
        .and(warp::path!("test" / "only" / "oracle"))
        .and(warp::get())
        .and(with_pool(pool.clone()))
        .and(with_credential_cache(credential_cache.clone()))
        .and_then(oracle::oracle);

    let routes = callback
        .or(login)
        .or(login_alt)
        .or(login_corporation)
        .or(whoami);

    #[cfg(feature = "test")]
    let routes = routes.or(oracle);

    routes.boxed()
}

/// Called after a character successfully logged in over at the EVE login page
///
/// # Params
///
/// * `service` -> Service for handling authentication
/// * `query`        -> Query params that come from the EVE servers after login
///
/// # Fails
///
/// Fails if the new user cannot be saved in the database
///
/// # Returns
///
/// Cookie containing a unique id of the logged in character and a redirect
/// to the main page of the webside
///
async fn callback(
    query:            HashMap<String, String>,
    credential_cache: Arc<Mutex<CredentialCache>>,
    pool:             PgPool,
) -> Result<Box<dyn warp::Reply>, Rejection> {
    let code = query
        .get("code")
        .ok_or(AuthError::InvalidCode)?;
    let state = query
        .get("state")
        .ok_or(AuthError::InvalidState)?;
    let state = Uuid::from_str(&state)
        .map_err(|_| AuthError::InvalidState)?;

    let character = callback::callback(
        &pool,
        code,
        &state,
        credential_cache
    ).await?;

    let redirect = std::env::var(REDIRECT)
        .unwrap_or_else(|_| String::from("http://localhost:1337"));

    // The character is an alt, so we don´t set a new cookie, we only redirect
    // the character back to the character page
    //if intention == Intention::LoginAlt || intention == Intention::LoginCorporation {
    if false {
        let redirect = format!("{}/characters", redirect);
        let response = warp::reply::with_header(
            warp::reply::with_status(
                warp::reply::json(&()),
                StatusCode::TEMPORARY_REDIRECT
            ), 
            LOCATION,
            redirect
        );
        return Ok(Box::new(response));
    }

    let token = generate_jwt(
        Claim {
            exp:          Utc::now()
                            .naive_utc()
                            .checked_add_months(Months::new(12))
                            .unwrap()
                            .and_utc()
                            .timestamp(),
            character_id: character
                            .character_id()
                            .map_err(|_| AuthError::InvalidCharacter)?,
        }
    )?;

    let cookie = format!(
        "token={}; Path=/; Secure; HttpOnly; Max-Age={}",
        token, 31557800 // 10 years
    );

    let response = warp::reply::with_header(
        warp::reply::with_header(
            warp::reply::with_status(
                warp::reply::json(&()),
                StatusCode::TEMPORARY_REDIRECT
            ),
            SET_COOKIE,
            cookie
        ),
        LOCATION,
        redirect
    );
    Ok(Box::new(response))
}

/// Login for a main account
///
/// # Params
///
/// * `eve_service` -> Service to handle eve authentication stuff
///
/// # Errors
///
/// Fails if a database operation is not successfull
///
/// # Returns
///
/// Redirect to the EVE login page
///
async fn login(
    pool: PgPool,
) -> Result<impl Reply, Rejection> {
    let url = login::login(&pool).await?;

    Ok(
        warp::reply::with_header(
            warp::reply::with_status(
                warp::reply::json(&()),
                StatusCode::TEMPORARY_REDIRECT
            ),
            LOCATION,
            url
        )
    )
}

/// Login for an alt character
///
/// # Params
///
/// * `service` -> Service to handle authentication
/// * `cookie`       -> Cookie of the currently logged in character
///
/// # Errors
///
/// Fails if the cookie is not in the database and any database operation for
/// login an alt fails
///
/// # Returns
///
/// Redirect to the EVE login page
///
async fn login_alt(
    pool:     PgPool,
    identity: Identity,
) -> Result<impl Reply, Rejection> {
    let character_id = identity.character_id();
    let url = login_alt::login_alt(&pool, character_id).await?;

    Ok(
        warp::reply::with_header(
            warp::reply::with_status(
                warp::reply::json(&()),
                StatusCode::TEMPORARY_REDIRECT
            ),
            LOCATION,
            url
        )
    )
}

async fn login_corporation(
    pool:     PgPool,
    identity: Identity,
) -> Result<impl Reply, Rejection> {
    let character_id = identity.character_id();
    let corporation_id = identity.corporation_id().await?;

    let url = login_corporation::login_corporation(
        &pool,
        character_id,
        corporation_id,
    ).await?;

    Ok(
        warp::reply::with_header(
            warp::reply::with_status(
                warp::reply::json(&()),
                StatusCode::TEMPORARY_REDIRECT
            ),
            LOCATION,
            url
        )
    )
}

async fn whoami(
    pool: PgPool,
    user: Identity,
) -> Result<impl Reply, Rejection> {
    let character_id = user.character_id();
    let client = user.api_client().await?;

    let whoami = crate::character::service::info(
            &pool,
            &client,
            character_id,
        )
        .await?;
    Ok(
        warp::reply::with_status(
            warp::reply::json(&whoami),
            StatusCode::OK,
        )
    )
}

#[derive(IntoParams)]
#[into_params(names("characterIdPath"))]
pub struct CharacterIdPath(pub CharacterId);
