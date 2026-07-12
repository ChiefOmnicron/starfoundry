use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use starfoundry_lib_gateway::ExtractIdentity;
use starfoundry_lib_industry::tag::Tag;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::AppState;
use crate::api_docs::{BadRequest, InternalServerError, Unauthorized};
use crate::tag::error::Result;
use crate::tag::service::list;

/// List Projects
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
/// - project_group:read
/// 
#[utoipa::path(
    get,
    path = "/",
    tag = "projects",
    params(TagFilter),
    responses(
        (
            body = Vec<Tag>,
            description = "List all projects that match the given filters",
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
    identity:      ExtractIdentity,
    State(state):  State<AppState>,
    Query(filter): Query<TagFilter>,
) -> Result<impl IntoResponse> {
    let data = list(
            &state.postgres,
            identity.character_id,
            filter,
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

#[derive(Debug, Default, Deserialize, Serialize, ToSchema, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct TagFilter {
    #[serde(default = "default_true")]
    #[param(
        default = json!(true),
        required = false,
    )]
    pub auto: Option<bool>,

    #[serde(default = "default_true")]
    #[param(
        default = json!(true),
        required = false,
    )]
    pub manual: Option<bool>,
}

fn default_true() -> Option<bool> {
    Some(true)
}
