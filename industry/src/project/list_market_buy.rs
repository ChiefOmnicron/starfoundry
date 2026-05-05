use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::Deserialize;
use starfoundry_lib_industry::project::ProjectMarketBuy;
use starfoundry_lib_industry::ProjectUuid;
use starfoundry_lib_market::{BuyStrategy, GasDecompressionEfficiency, OreReprocessingEfficiency};
use starfoundry_lib_types::StructureId;
use utoipa::{IntoParams, ToSchema};

use crate::{AppState, eve_gateway_api_client};
use crate::api_docs::{BadRequest, InternalServerError, NotFound, Unauthorized};
use crate::project::error::Result;
use crate::project::service::list_market_buy;

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
    post,
    path = "/{ProjectUuid}/market/buy",
    tag = "projects",
    params(
        ProjectUuid,
        ListMarketBuyQuery,
    ),
    responses(
        (
            body = Vec<ProjectMarketBuy>,
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
    State(state):     State<AppState>,
    Path(project_id): Path<ProjectUuid>,
    Json(config):     Json<ListMarketBuyQuery>,
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
    pub strategy:               BuyStrategy,
    pub structure_ids:          Vec<StructureId>,

    // gas decompression is active
    pub gas_decompression:      Option<GasDecompressionEfficiency>,
    // mineral compression is active
    pub mineral_compression:    Option<OreReprocessingEfficiency>,
}
