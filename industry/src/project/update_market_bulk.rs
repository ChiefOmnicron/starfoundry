use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use starfoundry_lib_industry::ProjectUuid;

use crate::api_docs::{Forbidden, InternalServerError, NotFound, Unauthorized, UnprocessableEntity, UnsupportedMediaType};
use crate::{AppState, eve_gateway_api_client, market_api_client};
use crate::project::error::Result;
use crate::project::service::{UpdateProjectMarket, update_market_bulk};
use utoipa::ToSchema;
use serde::Deserialize;
use starfoundry_lib_types::{StructureId, TypeId};
use starfoundry_lib_eve_gateway::EveGatewayApiClientItem;
use starfoundry_lib_market::{GasDecompressionEfficiency, OreReprocessingEfficiency};

/// Update Market
/// 
/// - Alternative route: `/v1/project/{ProjectUuid}/market`
/// - Alternative route: `/latest/project/{ProjectUuid}/market`
/// 
/// ---
/// 
/// Updates the market entries
/// 
/// ## Security
/// - authenticated
/// - project_group:write
/// 
#[utoipa::path(
    put,
    path = "/{ProjectUuid}/market",
    tag = "Project",
    request_body = Vec<UpdateProjectMarketRequest>,
    params(
        ProjectUuid,
    ),
    responses(
        (
            description = "The project was updated",
            status = NO_CONTENT,
        ),
        Unauthorized,
        Forbidden,
        NotFound,
        UnsupportedMediaType,
        UnprocessableEntity,
        InternalServerError,
    ),
    security(
        ("api_key" = [])
    ),
)]
pub async fn api(
    State(state):       State<AppState>,
    Path(project_id):   Path<ProjectUuid>,
    Json(update):       Json<UpdateProjectMarketRequest>,
) -> Result<impl IntoResponse> {
    let market_api_client = market_api_client()?;

    let entries = update
        .entries
        .iter()
        .filter(|x| x.type_id.is_some() || x.name.is_some())
        .collect::<Vec<_>>();

    let mut entries_with_type_id = entries
        .iter()
        .filter(|x| x.type_id.is_some())
        .map(|x| {
            UpdateProjectMarket {
                // unwrap is safe at this point, as it's validated above
                type_id:                x.type_id.unwrap(),
                cost:                   x.cost,
                quantity:               x.quantity,
                source:                 update.source.clone(),
                structure_id:           x.structure_id,
                gas_decompression:      update.gas_decompression,
                mineral_compression:    update.mineral_compression,
            }
        })
        .collect::<Vec<_>>();
    let names = entries
        .iter()
        .filter(|x| x.name.is_some())
        .collect::<Vec<_>>();
    let eve_gateway_api_client = eve_gateway_api_client()?;
    let parsed_items = eve_gateway_api_client
        .parse_items(
            names
                .iter()
                .map(|x| x.name.clone().unwrap_or_default())
                .collect::<Vec<_>>()
                .join("\n")
        )
        .await?;

    for parsed_item in parsed_items.items {
        let entry = if let Some(x) = names
            .iter()
            .find(|x| x.name.clone().unwrap_or_default() == parsed_item.item_name) {

            x
        } else {
            continue;
        };

        entries_with_type_id.push(UpdateProjectMarket {
            type_id:                parsed_item.type_id,
            cost:                   entry.cost,
            quantity:               entry.quantity,
            source:                 update.source.clone(),
            structure_id:           entry.structure_id,
            gas_decompression:      update.gas_decompression,
            mineral_compression:    update.mineral_compression,
        });
    }

    update_market_bulk(
        &state.postgres,
        project_id,
        entries_with_type_id,
        &market_api_client,
    ).await?;

    Ok((
        StatusCode::NO_CONTENT,
    ))
}

#[derive(Clone, Debug, Deserialize, ToSchema)]
pub struct UpdateProjectMarketRequest {
    pub source:                 String,
    pub entries:                Vec<UpdateProjectMarketEntry>,
    #[serde(default)]
    pub gas_decompression:      Option<GasDecompressionEfficiency>,
    #[serde(default)]
    pub mineral_compression:    Option<OreReprocessingEfficiency>,
}

#[derive(Clone, Debug, Deserialize, ToSchema)]
pub struct UpdateProjectMarketEntry {
    #[serde(default)]
    pub type_id:        Option<TypeId>,
    #[serde(default)]
    pub name:           Option<String>,
    #[serde(default)]
    pub structure_id:   Option<StructureId>,

    pub cost:           f32,
    pub quantity:       i32,
}
