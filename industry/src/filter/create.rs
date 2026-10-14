use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use sqlx::PgPool;
use starfoundry_lib_gateway::ExtractIdentity;
use starfoundry_lib_industry::filter::{CreateFilter, CreateFilterResponse};
use starfoundry_lib_types::CharacterId;

use crate::api_docs::{BadRequest, InternalServerError, Unauthorized};
use crate::AppState;
use crate::filter::error::{FilterError, Result};

/// Create Filter
/// 
/// - Alternative route: `/latest/filters`
/// - Alternative route: `/v1/filters`
/// 
/// ---
/// 
/// Creates a new project
/// 
/// ## Security
/// - authenticated
/// 
#[utoipa::path(
    post,
    path = "/",
    tag = "filters",
    request_body = CreateFilter,
    responses(
        (
            body = CreateFilterResponse,
            description = "Information about the new filter",
            status = OK,
        ),
        (
            description = "There aren't any projects matching the filter",
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
    identity:               ExtractIdentity,
    State(state):           State<AppState>,
    Json(create_filter):    Json<CreateFilter>,
) -> Result<impl IntoResponse> {
    let result = create(
            &state.postgres,
            identity.character_id,
            create_filter,
        )
        .await?;

    Ok(
        (
            StatusCode::CREATED,
            Json(result)
        )
    )
}

async fn create(
    pool:           &PgPool,
    character_id:   CharacterId,
    filter:         CreateFilter,
) -> Result<CreateFilterResponse> {
    let typ: String = filter.typ.into();
    let id = sqlx::query!("
            INSERT INTO filter
            (
                name,
                typ,
                filter,
                owner
            )
            VALUES ($1, $2, $3, $4)
            RETURNING id
        ",
            filter.name,
            typ,
            filter.filters,
            *character_id,
        )
        .fetch_one(pool)
        .await
        .map_err(FilterError::Create)?
        .id
        .into();

    Ok(CreateFilterResponse {
        id
    })
}
