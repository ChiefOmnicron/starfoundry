use sqlx::PgPool;
use starfoundry_libs_types::CharacterId;

use crate::{Error, ProjectGroupUuid, UpdateProjectGroupMember, Result};

pub async fn update_member(
    pool:        &PgPool,
    group_id:    ProjectGroupUuid,
    member_id:   CharacterId,
    permissions: UpdateProjectGroupMember,
) -> Result<()> {
    sqlx::query!("
            UPDATE project_group_members
            SET
                projects = $3,
                project_group = $4,
                structures = $5
            WHERE group_id = $1
              AND character_id = $2
        ",
            *group_id,
            *member_id,
            permissions.projects,
            permissions.project_group,
            permissions.structures,
        )
        .execute(pool)
        .await
        .map(drop)
        .map_err(|e| Error::UpdateGroupMember(e, group_id, member_id).into())
}
