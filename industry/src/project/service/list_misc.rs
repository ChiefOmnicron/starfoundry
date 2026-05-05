use sqlx::PgPool;
use starfoundry_lib_industry::project::ProjectMisc;
use starfoundry_lib_industry::ProjectUuid;

use crate::project::error::{ProjectError, Result};

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
