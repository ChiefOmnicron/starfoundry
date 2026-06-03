use axum::Json;
use axum::extract::{Query, State};
use axum::response::IntoResponse;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use starfoundry_lib_gateway::ExtractIdentity;
use utoipa::IntoParams;

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
    path = "/structure",
    tag = "Search",
    params(
        ("search" = String, Query),
    ),
    responses(
        (
            body = Vec<i64>,
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
    identity: ExtractIdentity,
    State(state): State<AppState>,
    Query(search_param): Query<SearchParam>,
) -> Result<impl IntoResponse> {
    if search_param.search.len() < 3 {
        return Err(SearchError::TooShort);
    }

    let api_client = api_client_auth(
        &state.postgres,
        state.eve_api_metric,
        identity.host()?,
        identity.character_id,
        vec![SCOPE.into()],
    )
    .await?;

    let api_client = if let Some(x) = api_client {
        x
    } else {
        return Ok((StatusCode::UNAUTHORIZED,).into_response());
    };

    #[derive(Serialize)]
    struct Query {
        categories: String,
        search: String,
    }

    let path = format!("latest/characters/{}/search", identity.character_id);
    let search_data = api_client
        .fetch_auth::<_, EveSearchResult>(
            &path,
            &Query {
                categories: "structure".into(),
                search: search_param.search,
            },
        )
        .await?;
    let mut ids: Vec<i64> = Vec::new();
    ids.extend(search_data.structure);

    if ids.is_empty() {
        return Ok((
            // TODO: use NO_CONTENT
            StatusCode::OK,
            Json(ids),
        )
        .into_response());
    }

    Ok((StatusCode::OK, Json(ids)).into_response())
}

/// The EVE-API returns some unfavorable data types, always using them will cause
/// more issues, so this type is a wrapper type to properly parse the EVE-API result
///
#[derive(Debug, Deserialize, IntoParams)]
pub struct SearchParam {
    pub search: String,
}

#[derive(Debug, Default, Deserialize)]
struct EveSearchResult {
    #[serde(default)]
    structure: Vec<i64>,
}
