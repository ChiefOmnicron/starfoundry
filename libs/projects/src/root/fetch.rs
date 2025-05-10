use sqlx::PgPool;

use crate::{Error, Project, ProjectStatus, ProjectUuid, Result};

pub async fn fetch(
    pool:         &PgPool,
    project_uuid: ProjectUuid,
) -> Result<Option<Project>> {
    let project = sqlx::query!(r#"
            SELECT
                name,
                status AS "status: ProjectStatus",
                orderer,
                note,
                structure_group_id,
                project_group_id
            FROM project
            WHERE id = $1
        "#,
            *project_uuid,
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::FetchProject(e, project_uuid))?;

    if let Some(x) = project {
        let finance = crate::finance::fetch(&pool, project_uuid)
            .await?
            .ok_or(Error::ProjectNotFound(project_uuid))?;

        let project = Project {
            name:               x.name,
            status:             x.status,
            structure_group_id: x.structure_group_id.into(),
            project_group_id:   x.project_group_id.into(),

            orderer:            x.orderer,
            note:               x.note,

            products:           crate::product::fetch(&pool, project_uuid).await?,
            finance,
        };
        Ok(Some(project))
    } else {
        Ok(None)
    }
}
