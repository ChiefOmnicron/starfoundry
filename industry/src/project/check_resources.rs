use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use starfoundry_lib_eve_gateway::EveGatewayApiClientItem;
use starfoundry_lib_gateway::{ErrorResponse, ExtractIdentity};
use starfoundry_lib_industry::project::{CheckMaterialsRequest, CheckMaterialsResponse, Material};

use crate::{AppState, eve_gateway_api_client, market_api_client};
use crate::api_docs::{BadRequest, InternalServerError, Unauthorized};
use crate::project::error::Result;
use crate::project::service::check_resources;

/// List Groups
/// 
/// - Alternative route: `/latest/projects/check`
/// - Alternative route: `/v1/projects/check`
/// 
/// ---
/// 
/// Checks the required materials
/// 
#[utoipa::path(
    post,
    path = "/check",
    tag = "projects",
    request_body = CheckMaterialsRequest,
    responses(
        (
            body = Vec<CheckMaterialsResponse>,
            description = "List of all required materials",
            status = OK,
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
    Json(info):     Json<CheckMaterialsRequest>,
) -> Result<impl IntoResponse> {
    let materials = if let Some(x) = info.materials {
        x
    } else if let Some(x) = info.materials_str {
        eve_gateway_api_client()?
            .parse_items(x)
            .await?
            .items
            .into_iter()
            .map(|x| Material {
                quantity: x.quantity as i32,
                type_id: x.type_id,
            })
            .collect::<Vec<_>>()
    } else {
        return Ok(
            (
                StatusCode::BAD_REQUEST,
                Json(
                    ErrorResponse {
                        error: "INVALID_REQUEST".into(),
                        description: "Either materials or materials_str must be set.".into(),
                    }
                )
            )
            .into_response()
        );
    };

    let data = check_resources(
            &state.postgres,
            &eve_gateway_api_client()?,
            &market_api_client()?,
            identity.character_id,
            info.job_ids,
            materials,
        ).await?;

    Ok(
        (
            StatusCode::OK,
            Json(data),
        )
        .into_response()
    )
}
