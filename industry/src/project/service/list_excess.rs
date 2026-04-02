use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use utoipa::ToSchema;

use crate::project::error::{ProjectError, Result};
use crate::project::ProjectUuid;
use starfoundry_lib_types::TypeId;

pub async fn list_excess(
    pool:       &PgPool,
    project_id: ProjectUuid,
) -> Result<Vec<ProjectExcess>> {
    let entries = sqlx::query!(r#"
            SELECT
                type_id,
                quantity
            FROM project_excess
            WHERE project_id = $1
        "#,
            *project_id,
        )
        .fetch_all(pool)
        .await
        .map_err(ProjectError::ListExcess)?
        .into_iter()
        .map(|x| ProjectExcess {
            type_id:    x.type_id.into(),
            quantity:   x.quantity,
        })
        .collect::<Vec<_>>();

    Ok(entries)
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct ProjectExcess {
    pub type_id:    TypeId,
    pub quantity:   i32,
}

