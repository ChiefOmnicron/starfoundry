use sqlx::PgPool;
use starfoundry_lib_types::CharacterId;

use crate::{Error, ProjectGroupUuid, Result};

#[deprecated]
pub async fn accept_member(
    pool:         &PgPool,
    character_id: CharacterId,
    group_id:     ProjectGroupUuid,
) -> Result<()> {
    sqlx::query!("
            UPDATE project_group_member
            SET
                accepted = TRUE
                --permission = 2
            WHERE group_id = $1
            AND character_id = $2
        ",
            *group_id,
            *character_id,
        )
        .execute(pool)
        .await
        .map(drop)
        .map_err(|e| Error::AcceptGroupMember(e, group_id).into())
}

