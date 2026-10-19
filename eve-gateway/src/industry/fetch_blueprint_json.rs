use axum::Json;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use starfoundry_lib_types::TypeId;
use utoipa::ToSchema;

use crate::api_docs::{InternalServerError, NotFound};
use crate::industry::error::{IndustryError, Result};
use crate::state::AppState;

/// Fetch Blueprint JSON
///
/// - Alternative route: `/latest/industry/blueprints/{TypeId}/json`
/// - Alternative route: `/v1/industry/blueprints/{TypeId}/json`
///
/// ---
///
/// Loads all open orders from a character
#[utoipa::path(
    get,
    path = "/blueprints/{TypeId}/json",
    tag = "Industry",
    responses(
        (
            body = BlueprintJson,
            description = "JSON representing blueprint dependencies",
            status = OK,
        ),
        NotFound,
        InternalServerError,
    ),
)]
pub async fn api(
    State(state): State<AppState>,
    Path(type_id): Path<TypeId>,
) -> Result<impl IntoResponse> {
    let blueprint_json = fetch_blueprint_json(&state.postgres, type_id).await?;

    if blueprint_json.is_none() {
        Ok((StatusCode::NO_CONTENT, Json(blueprint_json)).into_response())
    } else {
        Ok((StatusCode::OK, Json(blueprint_json)).into_response())
    }
}

async fn fetch_blueprint_json(
    pool: &PgPool,
    type_id: TypeId,
) -> Result<Option<BlueprintJson>> {
    sqlx::query!(
        r#"
            SELECT
                blueprint_type_id,
                product_type_id,
                data
            FROM blueprint_json
            WHERE blueprint_type_id = $1 OR product_type_id = $1
        "#,
        *type_id,
    )
    .fetch_optional(pool)
    .await
    .map(|x| {
        x.map(|y| {
            BlueprintJson {
                blueprint_type_id: y
                    .blueprint_type_id
                    .into(),
                product_type_id:   y
                    .product_type_id
                    .into(),
                data:              y.data,
            }
        })
    })
    .map_err(|e| IndustryError::FetchBlueprintJson(e, type_id))
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct BlueprintJson {
    pub blueprint_type_id: TypeId,
    pub product_type_id:   TypeId,
    pub data:              serde_json::Value,
}
