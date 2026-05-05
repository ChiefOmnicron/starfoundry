use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use starfoundry_lib_industry::{MarketUuid, ProjectUuid};
use starfoundry_lib_industry::project::UpdateMarketEntry;

use crate::api_docs::{Forbidden, InternalServerError, NotFound, Unauthorized};
use crate::AppState;
use crate::project::error::Result;
use crate::project::service::update_market_entry;

/// Delete market entry
/// 
/// - Alternative route: `/latest/projects/{ProjectUuid}/market/{MarketUuid}`
/// - Alternative route: `/v1/projects/{ProjectUuid}/market/{MarketUuid}`
/// 
/// ---
/// 
/// Deletes a market entry
/// 
/// ## Security
/// - authenticated
/// 
#[utoipa::path(
    put,
    path = "/{ProjectUuid}/market/{MarketUuid}",
    tag = "Project",
    request_body = UpdateMarketEntry,
    params(
        ProjectUuid,
    ),
    responses(
        (
            description = "Market entry was updated",
            status = NO_CONTENT,
        ),
        Unauthorized,
        Forbidden,
        NotFound,
        InternalServerError,
    ),
    security(
        ("api_key" = [])
    ),
)]
pub async fn api(
    State(state):                   State<AppState>,
    Path((project_id, market_id)):  Path<(ProjectUuid, MarketUuid)>,
    Json(update):                   Json<UpdateMarketEntry>,
) -> Result<impl IntoResponse> {
    update_market_entry(
            &state.postgres,
            project_id,
            market_id,
            update,
        )
        .await?;

    Ok(
        (
            StatusCode::NO_CONTENT,
            Json(())
        )
        .into_response()
    )
}
