use sqlx::PgPool;
use starfoundry_lib_eve_gateway::Blueprint;
use starfoundry_lib_types::{CharacterId, TypeId};
use crate::asset::{AssetError, Result};
use crate::item::services::load_items_by_type_id;

pub async fn list_blueprints(
    pool:         &PgPool,
    character_id: CharacterId,
) -> Result<Vec<Blueprint>> {
    let item_cache = if let Ok(x) = load_items_by_type_id(&pool).await {
        x
    } else {
        return Ok(Vec::new());
    };

    let blueprints = sqlx::query!(r#"
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

            item_id:             blueprint.item_id.into(),
            location_id:         blueprint.location_id.into(),
            location_flag:       blueprint.location_flag,
        });
    }

    Ok(result)
}
