use sqlx::PgPool;
use starfoundry_lib_eve_gateway::BlueprintJson;
use starfoundry_lib_types::TypeId;

use crate::industry::error::{IndustryError, Result};

pub async fn fetch_blueprint_json(
    pool:    &PgPool,
    type_id: TypeId,
) -> Result<Option<BlueprintJson>> {
    sqlx::query!(r#"
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
        .map(|x|
            x.map(|y| BlueprintJson {
                blueprint_type_id: y.blueprint_type_id.into(),
                product_type_id:   y.product_type_id.into(),
                data:              y.data,
            })
        )
        .map_err(|e| IndustryError::FetchBlueprintJson(e, type_id))
}
