use sqlx::PgPool;
use starfoundry_lib_industry::project::CreateProject;
use starfoundry_lib_industry::ProjectUuid;
use starfoundry_lib_types::CharacterId;

use crate::project::error::{ProjectError, Result};

pub async fn create(
    pool:         &PgPool,
    character_id: CharacterId,
    project_info: CreateProject,
) -> Result<ProjectUuid> {
    project_info.validate()?;

    let project_id = sqlx::query!(r#"
            INSERT INTO project
            (
                owner,
                sell_price,
                project_group_id,
                orderer,
                name,
                note,
                pre_products,
                pre_additional
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id
        "#,
            *character_id,
            project_info.sell_price,
            *project_info.project_group_id,
            project_info.orderer,
            project_info.name,
            project_info.notes,
            project_info.pre_products,
            project_info.pre_additional,
        )
        .fetch_one(pool)
        .await
        .map_err(ProjectError::Create)?;

    Ok(project_id.id.into())
}
