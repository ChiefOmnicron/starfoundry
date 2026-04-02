use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use starfoundry_lib_types::TypeId;
use utoipa::ToSchema;

use crate::project::error::{ProjectError, Result};
use crate::project::ProjectUuid;

pub async fn list_products(
    pool:       &PgPool,
    project_id: ProjectUuid,
) -> Result<Vec<ProjectProduct>> {
    let entries = sqlx::query!(r#"
            SELECT
                type_id,
                quantity,
                material_efficiency
            FROM solution_product sp
            JOIN project p ON p.solution_id = sp.solution_id
            WHERE p.id = $1
        "#,
            *project_id,
        )
        .fetch_all(pool)
        .await
        .map_err(ProjectError::ListProduct)?
        .into_iter()
        .map(|x| ProjectProduct {
            type_id:                x.type_id.into(),
            quantity:               x.quantity,
            material_efficiency:    x.quantity,
        })
        .collect::<Vec<_>>();

    Ok(entries)
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct ProjectProduct {
    pub type_id:             TypeId,
    pub quantity:            i32,
    pub material_efficiency: i32,
}

