use sqlx::PgPool;
use starfoundry_lib_eve_gateway::BlueprintDependency;
use starfoundry_lib_types::TypeId;

use crate::industry::error::{IndustryError, Result};

pub async fn fetch_blueprint_dependency_bulk(
    pool:     &PgPool,
    type_ids: Vec<TypeId>,
) -> Result<Vec<BlueprintDependency>> {
    sqlx::query!(r#"
            SELECT
                blueprint_type_id,
                product_type_id,
                time,
                depends_on
            FROM blueprint_dependency
            WHERE blueprint_type_id = ANY($1)
            OR product_type_id = ANY($1)
        "#,
            &type_ids.into_iter().map(|x| *x).collect::<Vec<_>>(),
        )
        .fetch_all(pool)
        .await
        .map(|x|
            x
                .into_iter()
                .map(|y| BlueprintDependency {
                    blueprint_type_id: y.blueprint_type_id.into(),
                    product_type_id:   y.product_type_id.into(),
                    time:              y.time,
                    depends_on:        y.depends_on.into_iter().map(Into::into).collect::<Vec<_>>(),
                })
                .collect::<Vec<_>>()
        )
        .map_err(IndustryError::FetchBlueprintDependency)
}
