use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use starfoundry_lib_types::TypeId;
use utoipa::ToSchema;

use crate::project_group::error::{ProjectGroupError, Result};
use crate::project_group::ProjectGroupUuid;

pub async fn update_default_blueprint_overwrite(
    pool:                   &PgPool,
    project_group_uuid:     ProjectGroupUuid,
    blueprint_overwrites:   Vec<UpdateProjectGroupDefaultBlueprintOverwrite>,
) -> Result<()> {
    let mut transaction = pool
        .begin()
        .await
        .map_err(ProjectGroupError::TransactionBeginError)?;

    sqlx::query!("
            DELETE FROM project_group_default_blueprint_overwrite
            WHERE project_group_id = $1
        ",
            *project_group_uuid,
        )
        .execute(&mut *transaction)
        .await
        .map_err(|e| ProjectGroupError::DeleteGroupDefaults(e, project_group_uuid))?;

    sqlx::query!("
            INSERT INTO project_group_default_blueprint_overwrite
            (
                project_group_id,
                type_id,
                material_efficiency
            )
            SELECT $1, * FROM UNNEST(
                $2::INTEGER[],
                $3::INTEGER[]
            )
        ",
            *project_group_uuid,
            &blueprint_overwrites.iter().map(|x| *x.type_id).collect::<Vec<_>>(),
            &blueprint_overwrites.iter().map(|x| x.material_efficiency).collect::<Vec<_>>(),
        )
        .execute(&mut *transaction)
        .await
        .map_err(|e| ProjectGroupError::UpdateGroupDefaults(e, project_group_uuid))?;

    transaction
        .commit()
        .await
        .map_err(ProjectGroupError::TransactionCommitError)
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(
    example = json!({
        "material_efficiency": 10,
        "type_id": 23773
    })
)]
pub struct UpdateProjectGroupDefaultBlueprintOverwrite {
    pub material_efficiency: i32,
    pub type_id:             TypeId,
}

#[cfg(test)]
mod update_default_blueprint_overwrite_project_group_test {
    use sqlx::PgPool;
    use starfoundry_lib_types::TypeId;
    use std::str::FromStr;
    use uuid::Uuid;

    use crate::project_group::service::UpdateProjectGroupDefaultBlueprintOverwrite;

    #[sqlx::test(
        fixtures(
            path = "../fixtures",
            scripts("base"),
        ),
    )]
    async fn happy_path(
        pool: PgPool,
    ) {
        let response = super::update_default_blueprint_overwrite(
                &pool,
                Uuid::from_str("00000000-0000-0000-0000-000000000001").unwrap().into(),
                vec![UpdateProjectGroupDefaultBlueprintOverwrite {
                    type_id: TypeId(1),
                    material_efficiency: 1,
                }],
            )
            .await;
        assert!(response.is_ok());
    }
}
