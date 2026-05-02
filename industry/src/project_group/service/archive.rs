use sqlx::PgPool;
use starfoundry_lib_industry::ProjectGroupUuid;

use crate::project_group::error::{ProjectGroupError, Result};

pub async fn archive(
    pool:     &PgPool,
    group_id: ProjectGroupUuid,
) -> Result<()> {
    sqlx::query!("
        UPDATE project_group
        SET archived = NOT archived
        WHERE id = $1
    ",
        *group_id,
    )
    .execute(pool)
    .await
    .map(drop)
    .map_err(|e| ProjectGroupError::ArchiveGroup(e, group_id).into())
}
