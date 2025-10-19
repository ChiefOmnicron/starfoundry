use sqlx::PgPool;
use starfoundry_lib_types::CharacterId;

use crate::{Error, ProjectGroupPermission, ProjectGroupUuid, Result};

#[deprecated]
pub async fn update_member_permission(
    pool:        &PgPool,
    group_id:    ProjectGroupUuid,
    member_id:   CharacterId,
    permissions: ProjectGroupPermission,
) -> Result<()> {
    unimplemented!()
    /*sqlx::query!("
            UPDATE project_group_member
            SET permission = $3
            WHERE group_id = $1
              AND character_id = $2
        ",
            *group_id,
            *member_id,
            *permissions,
        )
        .execute(pool)
        .await
        .map(drop)
        .map_err(|e| Error::UpdateGroupMember(e, group_id, member_id).into())*/
}
