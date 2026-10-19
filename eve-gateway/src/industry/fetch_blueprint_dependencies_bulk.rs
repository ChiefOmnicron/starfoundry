use axum::Json;
use axum::extract::State;
use axum::response::IntoResponse;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use starfoundry_lib_types::TypeId;
use utoipa::ToSchema;

use crate::api_docs::{InternalServerError, NotFound};
use crate::industry::error::{IndustryError, Result};
use crate::state::AppState;

/// Fetch Blueprint Dependencies
///
/// - Alternative route: `/latest/industry/blueprints/dependencies/bulk`
/// - Alternative route: `/v1/industry/blueprints/dependencies/bulk`
///
/// ---
///
/// Loads all open orders from a character
#[utoipa::path(
    post,
    path = "/blueprints/dependencies/bulk",
    tag = "Industry",
    responses(
        (
            body = Vec<BlueprintDependency>,
            description = "Blueprint and their dependencies",
            status = OK,
        ),
        NotFound,
        InternalServerError,
    ),
)]
pub async fn api(
    State(state): State<AppState>,
    Json(type_ids): Json<Vec<TypeId>>,
) -> Result<impl IntoResponse> {
    let blueprint_dependencies =
        fetch_blueprint_dependency_bulk(&state.postgres, type_ids).await?;

    if blueprint_dependencies.is_empty() {
        Ok((StatusCode::NO_CONTENT, Json(blueprint_dependencies))
            .into_response())
    } else {
        Ok((StatusCode::OK, Json(blueprint_dependencies)).into_response())
    }
}

async fn fetch_blueprint_dependency_bulk(
    pool: &PgPool,
    type_ids: Vec<TypeId>,
) -> Result<Vec<BlueprintDependency>> {
    sqlx::query!(
        r#"
            SELECT
                blueprint_type_id,
                product_type_id,
                time,
                depends_on
            FROM blueprint_dependency
            WHERE blueprint_type_id = ANY($1)
            OR product_type_id = ANY($1)
        "#,
        &type_ids
            .into_iter()
            .map(|x| *x)
            .collect::<Vec<_>>(),
    )
    .fetch_all(pool)
    .await
    .map(|x| {
        x.into_iter()
            .map(|y| {
                BlueprintDependency {
                    blueprint_type_id: y
                        .blueprint_type_id
                        .into(),
                    product_type_id:   y
                        .product_type_id
                        .into(),
                    time:              y.time,
                    depends_on:        y
                        .depends_on
                        .into_iter()
                        .map(Into::into)
                        .collect::<Vec<_>>(),
                }
            })
            .collect::<Vec<_>>()
    })
    .map_err(IndustryError::FetchBlueprintDependency)
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct BlueprintDependency {
    pub blueprint_type_id: TypeId,
    pub product_type_id:   TypeId,
    pub time:              i32,
    pub depends_on:        Vec<TypeId>,
}
