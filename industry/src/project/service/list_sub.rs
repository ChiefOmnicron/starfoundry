use sqlx::PgPool;
use starfoundry_lib_industry::project::{ProjectMinimal, ProjectStatus};
use starfoundry_lib_industry::ProjectUuid;
use starfoundry_lib_types::CharacterId;
use std::collections::HashMap;

use crate::project::error::{ProjectError, Result};
use crate::project_group::service::ProjectGroupFilter;

pub async fn list_sub_projects(
    pool:           &PgPool,
    character_id:   CharacterId,
    project_id:     ProjectUuid,
) -> Result<Option<Vec<ProjectMinimal>>> {
    let sub_projects = sqlx::query!(r#"
            SELECT
                id,
                name,
                status AS "status: ProjectStatus",
                orderer,
                sell_price,
                project_group_id
            FROM project_sub_project psp
            JOIN project p ON psp.sub_project_id = p.id
            WHERE psp.project_id = $1
        "#,
            *project_id,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| ProjectError::Fetch(e, project_id))?;

    let project_groups = crate::project_group::service::list(
            pool,
            character_id,
            ProjectGroupFilter::default(),
        )
        .await?
        .into_iter()
        .map(|x| (x.id, x))
        .collect::<HashMap<_, _>>();

    let mut result = Vec::new();
    for sub_project in sub_projects {
        let project_group = if let Some(x) = project_groups.get(&sub_project.project_group_id.into()) {
            x.clone()
        } else {
            continue;
        };

        let project_group = ProjectMinimal {
            id:            sub_project.id.into(),
            name:          sub_project.name,
            status:        sub_project.status,
            orderer:       sub_project.orderer,
            sell_price:    sub_project.sell_price,
            project_group: project_group,
        };
        result.push(project_group);
    }

    if result.is_empty() {
        Ok(None)
    } else {
        Ok(Some(result))
    }
}
