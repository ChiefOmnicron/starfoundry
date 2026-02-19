use axum::extract::{Query, State};
use axum::Json;
use axum::response::IntoResponse;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use starfoundry_lib_gateway::ExtractIdentity;
use utoipa::{IntoParams, ToSchema};

use crate::api_docs::{BadRequest, InternalServerError, Unauthorized};
use crate::search::{Result, SearchError};
use crate::state::AppState;
use crate::utils::api_client_auth;

const SCOPE: &str = "esi-search.search_structures.v1";

/// Search
/// 
/// - Alternative route: `/latest/search`
/// - Alternative route: `/v1/search`
/// 
/// ---
/// 
/// Searches for the given string and category
/// 
#[utoipa::path(
    get,
    path = "/",
    tag = "Search",
    params(
        ("categories" = String, Query),
        ("search" = String, Query),
    ),
    responses(
        (
            body = Vec<IdToName>,
            description = "List of results",
            status = OK,
        ),
        (
            description = "Nothing was found",
            status = NO_CONTENT,
        ),
        BadRequest,
        InternalServerError,
        Unauthorized,
    ),
)]
pub async fn api(
    identity:            ExtractIdentity,
    State(state):        State<AppState>,
    Query(search_param): Query<SearchParam>,
) -> Result<impl IntoResponse> {
    if search_param.search.len() < 3 {
        return Err(SearchError::TooShort);
    }

    let api_client = api_client_auth(
            &state.postgres,
            identity.host()?,
            identity.character_id,
            vec![
                SCOPE.into(),
            ],
        )
        .await?;

    let api_client = if let Some(x) = api_client {
        x
    } else {
        return Ok(
            (
                StatusCode::UNAUTHORIZED,
            )
            .into_response()
        )
    };

    #[derive(Serialize)]
    struct Query {
        categories: String,
        search:     String,
    }

    let path = format!("latest/characters/{}/search", identity.character_id);
    let search_data = api_client
        .fetch_auth::<_, EveSearchResult>(
            &path,
            &Query {
                categories: search_param.categories.clone(),
                search:     search_param.search,
            }
        )
        .await?;
    let mut ids = Vec::new();
    ids.extend(search_data.alliance);
    ids.extend(search_data.corporation);
    ids.extend(search_data.character);

    if ids.is_empty() {
        return Ok(
            (
                StatusCode::NO_CONTENT,
                Json(ids),
            )
            .into_response()
        )
    }

    let entries = api_client
        .post::<Vec<i64>, Vec<IdToName>>(
            ids,
            "/universe/names"
        )
        .await?;

    Ok(
        (
            StatusCode::OK,
            Json(entries),
        )
        .into_response()
    )
}

/// The EVE-API returns some unfavorable data types, always using them will cause
/// more issues, so this type is a wrapper type to properly parse the EVE-API result
/// 
#[derive(Debug, Deserialize, IntoParams)]
pub struct SearchParam {
    pub search:     String,

    #[param(
        default = json!("alliance,character,corporation"),
        required = true,
    )]
    pub categories: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct IdToName {
    pub id:       i64,
    /// Allowed values: alliance, character, constellation, corporation, inventory_type, region, solar_system, station, faction
    pub category: String,
    pub name:     String,
}

#[derive(Default, Deserialize)]
struct EveSearchResult {
    #[serde(default)]
    alliance:       Vec<i64>,
    #[serde(default)]
    character:      Vec<i64>,
    #[serde(default)]
    corporation:    Vec<i64>,
}
