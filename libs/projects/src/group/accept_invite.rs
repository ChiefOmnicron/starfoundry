use sqlx::PgPool;
use starfoundry_libs_types::CharacterId;

use crate::{Error, ProjectGroupUuid, Result};

pub async fn accept_invite(
    pool:         &PgPool,
    character_id: CharacterId,
    group_id:     ProjectGroupUuid,
) -> Result<()> {
    sqlx::query!("
        INSERT INTO project_group_members (
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

