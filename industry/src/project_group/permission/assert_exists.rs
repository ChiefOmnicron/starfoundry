use sqlx::PgPool;

use crate::project_group::ProjectGroupUuid;
use crate::project_group::error::{ProjectGroupError, Result};

pub async fn assert_exists(
    pool:               &PgPool,
    project_group_uuid: ProjectGroupUuid,
) -> Result<()> {
    let project = sqlx::query!("
            SELECT id
            FROM project_group
            WHERE id = $1
        ",
            *project_group_uuid,
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| ProjectGroupError::FetchGroup(e, project_group_uuid))?;

    if project.is_some() {
        Ok(())
    } else {
        Err(ProjectGroupError::NotFound(project_group_uuid))
    }
}
