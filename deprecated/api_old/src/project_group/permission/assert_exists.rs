use sqlx::PgPool;

use crate::project_group::ProjectGroupUuid;
use crate::project_group::error::{Error, Result};

pub async fn assert_exists(
    pool:               &PgPool,
    project_group_uuid: ProjectGroupUuid,
) -> Result<Option<()>> {
    let project = sqlx::query!("
            SELECT id
            FROM project_group
            WHERE id = $1
        ",
            *project_group_uuid,
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::FetchGroup(e, project_group_uuid))?;

    if project.is_some() {
        Ok(Some(()))
    } else {
        Err(Error::NotFound(project_group_uuid))
    }
}
