use chrono::{Months, Utc};
use sqlx::PgPool;
use starfoundry_lib_eve_api::{CredentialCache, EveApiClient};
use starfoundry_lib_types::CorporationId;
use std::sync::{Arc, Mutex};
use warp::{Reply, Rejection};
use warp::http::header::{LOCATION, SET_COOKIE};
use warp::http::StatusCode;

use super::{generate_jwt, Claim};

/// Url to redirect after login
const REDIRECT: &str = "REDIRECT";

pub async fn oracle(
    pool:             PgPool,
    credential_cache: Arc<Mutex<CredentialCache>>,
) -> Result<Box<dyn Reply>, Rejection> {
    // make sure the rng is out of scope before using await, otherwise
    // it will not be Send
    let character_id = {
        //let mut rng = rand::thread_rng();
        //let character_id = rng.gen_range(0..100);
        //character_id.clone()
        2118649097
    };

    sqlx::query!("
            DELETE FROM character
            WHERE character_id = $1
        ",
            character_id,
        )
        .execute(&pool)
        .await
        .unwrap();

    let redirect = std::env::var(REDIRECT).unwrap();
    let token = generate_jwt(
        Claim {
            exp:          Utc::now()
                            .naive_utc()
                            .checked_add_months(Months::new(12))
                            .unwrap()
                            .and_utc()
                            .timestamp(),
            character_id: character_id.into()
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
                StatusCode::TEMPORARY_REDIRECT,
            ),
            SET_COOKIE,
            cookie
        ),
        LOCATION,
        redirect
    );

    {
        let eve_client = EveApiClient::new_with_refresh_token(
            character_id.into(),
            CorporationId(1),
            String::new(),
        ).unwrap();

        let mut client = {
            credential_cache
                .lock()
                .unwrap()
        };

        client.insert(character_id.into(), eve_client);
    }

    Ok(Box::new(response))
}
