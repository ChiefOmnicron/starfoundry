use axum::extract::{Query, State};
use axum::Json;
use axum::response::IntoResponse;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use starfoundry_lib_gateway::ExtractIdentity;

use crate::api_docs::{InternalServerError, NotFound, Unauthorized};
use crate::market::error::Result;
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
        ("category" = String, Query),
        ("search" = String, Query),
    ),
    responses(
        (
            body = Vec<i32>,
            description = "List of results",
            status = OK,
        ),
        (
            description = "Nothing was found",
            status = NO_CONTENT,
        ),
        NotFound,
        InternalServerError,
        Unauthorized,
    ),
)]
pub async fn api(
    identity:            ExtractIdentity,
    State(state):        State<AppState>,
    Query(search_param): Query<SearchParam>,
) -> Result<impl IntoResponse> {
    dbg!(&search_param);
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
    let category: String = search_param.category.into();
    let search_data = api_client
        .fetch_auth::<_, EveSearchResult>(
            &path,
            &Query {
                categories: category.clone(),
                search:     search_param.search,
            }
        )
        .await?;

    let result = if category == "character" {
        let characters = crate::character::fetch_bulk(
                &state.postgres,
                search_data
                    .character
                    .into_iter()
                    .map(Into::into)
                    .collect::<Vec<_>>(),
            )
            .await
            .unwrap();
        serde_json::to_value(&characters).unwrap()
    } else {
        serde_json::json!({})
    };

    Ok(
        (
            StatusCode::OK,
            Json(result),
        )
        .into_response()
    )
}

/// The EVE-API returns some unfavorable data types, always using them will cause
/// more issues, so this type is a wrapper type to properly parse the EVE-API result
/// 
#[derive(Debug, Deserialize)]
pub struct SearchParam {
    search:   String,
    //category: SearchCategory,
    category: String,
}

pub struct SearchResult(pub Vec<i64>);

#[derive(Deserialize)]
struct EveSearchResult {
    //#[serde(default)]
    //alliance:       Vec<i64>,
    #[serde(default)]
    character:      Vec<i32>,
    //#[serde(default)]
    //corporation:    Vec<i64>,
}
