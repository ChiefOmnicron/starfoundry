use sqlx::PgPool;
use starfoundry_lib_types::CharacterId;

use crate::project_group::permission::ProjectGroupPermissionCode;
use crate::project_group::ProjectGroupUuid;
use crate::project_group::error::{Error, Result};

pub async fn assert_read_access(
    pool:               &PgPool,
    project_group_uuid: ProjectGroupUuid,
    character_id:       CharacterId,
) -> Result<()> {
    let result = sqlx::query!("
            SELECT pg.id
            FROM project_group pg
            JOIN project_group_member pgm ON pgm.group_id = pg.id
            WHERE pg.id = $1
            AND pgm.character_id = $2
            AND (
                permission & $3 = $3 OR
                permission & $4 = $4
            )
        ",
            *project_group_uuid,
            *character_id,
            *ProjectGroupPermissionCode::Owner,
            *ProjectGroupPermissionCode::ReadGroup,
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::FetchGroupPermissions(e, project_group_uuid))?;

    if result.is_none() {
        return Err(Error::Forbidden(project_group_uuid, character_id));
    } else {
        Ok(())
    }
}
