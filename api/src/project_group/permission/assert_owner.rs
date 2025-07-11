use sqlx::PgPool;
use starfoundry_libs_types::CharacterId;

use crate::project_group::ProjectGroupUuid;
use crate::project_group::error::{ProjectGroupError, Result};

pub async fn assert_owner(
    pool:               &PgPool,
    project_group_uuid: ProjectGroupUuid,
    character_id:       CharacterId,
) -> Result<()> {
    let result = sqlx::query!("
            SELECT id
            FROM project_group
            WHERE id = $1
            AND owner = $2
        ",
            *project_group_uuid,
            *character_id,
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| ProjectGroupError::FetchGroupPermissions(e, project_group_uuid))?;

    if result.is_none() {
        return Err(ProjectGroupError::Forbidden(project_group_uuid, character_id));
    } else {
        Ok(())
    }   
}
