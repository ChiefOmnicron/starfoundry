use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use starfoundry_lib_gateway::ExtractIdentity;
use starfoundry_lib_market::BuyStrategy;
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};

use crate::{AppState, eve_gateway_api_client};
use crate::api_docs::{BadRequest, InternalServerError, NotFound, Unauthorized};
use crate::project::error::Result;
use crate::project::ProjectUuid;
use crate::project::service::{ProjectMarket, list_market_buy};

/// List Market
/// 
/// - Alternative route: `/latest/projects/{ProjectUuid}/market/buy`
/// - Alternative route: `/v1/projects/{ProjectUuid}/market/buy`
/// 
/// ---
/// 
/// Lists all materials that need to be bought
/// 
/// ## Security
/// - authenticated
/// - project:read
/// 
#[utoipa::path(
    get,
    path = "/{ProjectUuid}/market/buy",
    tag = "projects",
    params(
        ProjectUuid,
        ListMarketBuyQuery,
    ),
    responses(
        (
            body = Vec<ProjectMarket>,
            description = "List all materials for a project",
            status = OK,
        ),
        (
            description = "There aren't any materials required for the project",
            status = NO_CONTENT,
        ),
        NotFound,
        BadRequest,
        Unauthorized,
        InternalServerError,
    ),
    security(
        ("api_key" = [])
    ),
)]
pub async fn api(
    _identity:        ExtractIdentity,
    State(state):     State<AppState>,
    Path(project_id): Path<ProjectUuid>,
    Query(config):    Query<ListMarketBuyQuery>,
) -> Result<impl IntoResponse> {
    let data = list_market_buy(
            &state.postgres,
            project_id,
            &eve_gateway_api_client()?,
            config,
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

#[derive(Debug, Default, Deserialize, ToSchema, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct ListMarketBuyQuery {
    pub strategy: BuyStrategy,
}
