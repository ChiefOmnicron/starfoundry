use std::sync::Arc;

use chrono::NaiveDateTime;
use reqwest::Response;
use serde::{Deserialize, Deserializer};
use sqlx::PgPool;
use starfoundry_lib_eve_client::{
    EveApiClient, EveApiClientMetric, EveApiError, Result
};
use starfoundry_lib_types::CharacterId;

pub async fn api_client_auth(
    pool: &PgPool,
    metric: Arc<EveApiClientMetric>,
    host: String,
    character_id: CharacterId,
    min_scopes: Vec<String>,
) -> Result<Option<EveApiClient>> {
    let result = sqlx::query!(
        "
            SELECT refresh_token
            FROM eve_credential
            WHERE
                domain = $1 AND
                (
                    character_id = $2
                ) AND
                scopes && $3::VARCHAR[]
        ",
        host,
        *character_id,
        &min_scopes,
    )
    .fetch_optional(pool)
    .await
    .map_err(|_| EveApiError::NoIdentity)?;

    let refresh_token = if let Some(x) = result {
        x.refresh_token
    } else {
        tracing::warn!(
            "No character found with scope {} for {}",
            min_scopes.join(","),
            *character_id
        );
        return Ok(None)
    };

    EveApiClient::new_with_refresh_token(metric, character_id, refresh_token)
        .map(Option::Some)
}

pub fn from_datetime<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let datetime: String = Deserialize::deserialize(deserializer)?;
    let datetime =
        NaiveDateTime::parse_from_str(&datetime, "%Y-%m-%dT%H:%M:%S%Z")
            .unwrap_or_default();

    Ok(datetime)
}

/// Extract the page header from the give [reqwest::Response].
///
/// # Params
///
/// * `response` -> Response to get the header from
///
/// # Returns
///
/// - If the header is not present a 0 is returned
/// - If the header exists, it will try to parse it, if that fails a 0 is is
///   returned
pub fn page_count(response: &Response) -> u16 {
    let headers = response.headers();
    if let Some(x) = headers.get("x-pages") {
        x.to_str()
            .unwrap_or_default()
            .parse::<u16>()
            .unwrap_or_default()
    } else {
        0u16
    }
}

/// Checks if the return has content in it.
///
/// # Params
///
/// * `response` -> Response to get the header from
///
/// # Returns
///
/// - False if the header does not exists
/// - False if the content does not contain data True if the content contains
///   data
pub fn has_content(response: &Response) -> bool {
    let headers = response.headers();
    if let Some(x) = headers.get("content-length") {
        x.to_str()
            .unwrap_or_default()
            .parse::<u64>()
            .unwrap_or_default()
            > 0
    } else {
        false
    }
}
