use sqlx::PgPool;
use starfoundry_lib_types::CharacterId;

use crate::{Error, ProjectGroupUuid, Result};

#[deprecated]
pub async fn remove_member(
    pool:             &PgPool,
    project_group_id: ProjectGroupUuid,
    member_id:        CharacterId,
) -> Result<()> {
    sqlx::query!("
        DELETE FROM project_group_member
        WHERE group_id = $1
        AND character_id = $2
    ",
        *project_group_id,
        *member_id,
    )
    .execute(pool)
    .await
    .map(drop)
    .map_err(|e|
        Error::RemoveGroupMember(e, project_group_id, member_id).into()
    )
}

