use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use sqlx::PgPool;
use starfoundry_lib_types::{CharacterId, TypeId};

use crate::api_docs::{InternalServerError, NotFound};
use crate::asset::Blueprint;
use crate::asset::error::{AssetError, Result};
use crate::item::load_items_by_type_id;
use crate::state::AppState;

/// List Blueprints
///
/// - Alternative route: `/latest/assets/blueprints`
/// - Alternative route: `/v1/assets/blueprints`
///
/// ---
///
/// Lists all blueprints the character has access to. This includes for all
/// characters and corporations the main character is linked to
#[utoipa::path(
    get,
    path = "/blueprint",
    tag = "Assets",
    params(
        CharacterId,
    ),
    responses(
        (
            body = Vec<Blueprint>,
            description = "List of all blueprints the character has access to",
            status = OK,
        ),
        NotFound,
        InternalServerError,
    ),
)]
pub async fn api(
    State(state): State<AppState>,
    Path(character_id): Path<CharacterId>,
) -> Result<impl IntoResponse> {
    let entry = list_blueprints(&state.postgres, character_id).await?;

    Ok((StatusCode::OK, Json(entry)).into_response())
}

async fn list_blueprints(
    pool: &PgPool,
    character_id: CharacterId,
) -> Result<Vec<Blueprint>> {
    let item_cache = load_items_by_type_id();

    let blueprints = sqlx::query!(
        r#"
            SELECT
                item_id,
                location_id,
                location_flag,
                type_id,
                material_efficiency,
                time_efficiency,
                quantity,
                runs
            FROM asset_blueprint
            WHERE owner_id = $1
        "#,
        *character_id,
    )
    .fetch_all(pool)
    .await
    .map_err(AssetError::ListBlueprints)?;

    let mut result = Vec::new();
    for blueprint in blueprints {
        let item = if let Some(x) = item_cache.get(&TypeId(blueprint.type_id)) {
            x
        } else {
            continue;
        };

        result.push(Blueprint {
            item:                item.clone(),
            material_efficiency: blueprint.material_efficiency,
            time_efficiency:     blueprint.time_efficiency,
            quantity:            blueprint.quantity,
            runs:                blueprint.runs,

            item_id:       blueprint
                .item_id
                .into(),
            location_id:   blueprint
                .location_id
                .into(),
            location_flag: blueprint.location_flag,
        });
    }

    Ok(result)
}
