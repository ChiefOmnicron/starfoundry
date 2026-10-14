use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use sqlx::PgPool;
use starfoundry_lib_gateway::ExtractIdentity;
use starfoundry_lib_industry::filter::{Filter, FilterQuery, FilterType};
use starfoundry_lib_types::CharacterId;

use crate::AppState;
use crate::api_docs::{BadRequest, InternalServerError, Unauthorized};
use crate::filter::error::{FilterError, Result};

/// List Filter
/// 
/// - Alternative route: `/latest/projects`
/// - Alternative route: `/v1/projects`
/// 
/// ---
/// 
/// Lists all projects the user has access to.
/// 
/// ## Security
/// - authenticated
/// - owner
/// 
#[utoipa::path(
    get,
    path = "/",
    tag = "filters",
    params(FilterQuery),
    responses(
        (
            body = Vec<Filter>,
            description = "List all filters",
            status = OK,
        ),
        (
            description = "There aren't any filters",
            status = NO_CONTENT,
        ),
        BadRequest,
        Unauthorized,
        InternalServerError,
    ),
    security(
        ("api_key" = [])
    ),
)]
pub async fn api(
    identity:       ExtractIdentity,
    State(state):   State<AppState>,
    Query(query):   Query<FilterQuery>,
) -> Result<impl IntoResponse> {
    let data = list(
            &state.postgres,
            identity.character_id,
            query.typ,
        ).await?;

    if data.is_empty() {
        Ok(
            (
                StatusCode::NO_CONTENT,
                Json(data),
            )
            .into_response()
        )
    } else {
        Ok(
            (
                StatusCode::OK,
                Json(data),
            )
            .into_response()
        )
    }
}

async fn list(
    pool:           &PgPool,
    character_id:   CharacterId,
    typ:            FilterType,
) -> Result<Vec<Filter>> {
    let typ_filter: String = typ.into();
    let filters = sqlx::query!("
            SELECT
                id,
                name,
                filter
            FROM filter
            WHERE owner = $1
            AND typ = $2
        ",
            *character_id,
            typ_filter,
        )
        .fetch_all(pool)
        .await
        .map_err(FilterError::List)?
        .into_iter()
        .map(|x| Filter {
            id:         x.id.into(),
            filters:    x.filter,
            name:       x.name,
        })
        .collect::<Vec<_>>();
    Ok(filters)
}
