use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::project::error::{ProjectError, Result};
use crate::project::ProjectUuid;

pub async fn list_misc(
    pool:       &PgPool,
    project_id: ProjectUuid,
) -> Result<Vec<ProjectMisc>> {
    let entries = sqlx::query!(r#"
            SELECT
                id,
                item,
                cost,
                quantity,
                description
            FROM project_misc
            WHERE project_id = $1
        "#,
            *project_id,
        )
        .fetch_all(pool)
        .await
        .map_err(ProjectError::ListMisc)?
        .into_iter()
        .map(|x| ProjectMisc {
            id:          x.id,
            item:        x.item,
            cost:        x.cost,

            description: x.description,
            quantity:    x.quantity,
        })
        .collect::<Vec<_>>();

    Ok(entries)
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct ProjectMisc {
    pub id:          Uuid,
    pub item:        String,
    pub cost:        f64,

    pub description: Option<String>,
    pub quantity:    Option<i32>,
}

