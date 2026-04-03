use serde::Deserialize;
use sqlx::PgPool;
use starfoundry_lib_types::CharacterId;
use utoipa::ToSchema;

use crate::project_group::error::{ProjectGroupError, Result};
use crate::project_group::ProjectGroupUuid;
use crate::project_group::permission::ProjectGroupPermissionCode;

pub async fn update_member(
    pool:               &PgPool,
    project_group_uuid: ProjectGroupUuid,
    update_info:        Vec<UpdateMemberRequest>,
) -> Result<()> {
    let mut transaction = pool
        .begin()
        .await
        .map_err(ProjectGroupError::TransactionBeginError)?;

    sqlx::query!("
            DELETE FROM project_group_member
            WHERE project_group_id = $1
        ",
            *project_group_uuid,
        )
        .execute(&mut *transaction)
        .await
        .map_err(|e| ProjectGroupError::DeleteIndustryHubs(e, project_group_uuid))?;

    sqlx::query!("
            INSERT INTO project_group_member
            (
                project_group_id,
                permission,
                character_id
            )
            SELECT $1, $2, * FROM UNNEST(
                $3::INTEGER[]
            )
            ON CONFLICT (project_group_id, character_id) DO NOTHING
        ",
            *project_group_uuid,
            // TODO: proper permission
            *ProjectGroupPermissionCode::Owner,
            &update_info.into_iter().map(|x| *x.character_id).collect::<Vec<_>>(),
        )
        .execute(&mut *transaction)
        .await
        .map_err(|e| ProjectGroupError::UpdateMembers(e, project_group_uuid))?;

    transaction
        .commit()
        .await
        .map_err(ProjectGroupError::TransactionCommitError)
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateMemberRequest {
    character_id: CharacterId,
}
