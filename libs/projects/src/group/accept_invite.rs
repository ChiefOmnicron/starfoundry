use sqlx::PgPool;
use starfoundry_lib_types::CharacterId;

use crate::{Error, ProjectGroupUuid, Result};

#[deprecated]
pub async fn accept_invite(
    pool:         &PgPool,
    character_id: CharacterId,
    group_id:     ProjectGroupUuid,
) -> Result<()> {
    sqlx::query!("
        INSERT INTO project_group_member(
            group_id,
            character_id
        )
        VALUES ($1, $2)
        ON CONFLICT (group_id, character_id)
        DO NOTHING
    ",
        *group_id,
        *character_id,
    )
    .execute(pool)
    .await
    .map(drop)
    .map_err(|e| Error::AcceptGroupInvite(e, group_id).into())
}

