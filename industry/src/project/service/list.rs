use sqlx::PgPool;
use starfoundry_lib_industry::project::{ProjectFilter, ProjectMinimal, ProjectStatus};
use starfoundry_lib_types::CharacterId;
use std::collections::HashMap;

use crate::project::error::{ProjectError, Result};
use crate::project_group::service::ProjectGroupFilter;

pub async fn list(
    pool:           &PgPool,
    character_id:   CharacterId,
    filter:         ProjectFilter,
) -> Result<Vec<ProjectMinimal>> {
    let user_project_groups = crate::project_group::service::list(
            pool,
            character_id,
            Default::default()
        )
        .await?
        .into_iter()
        .map(|x| *x.id)
        .collect::<Vec<_>>();

    let filter_status: Vec<String> = if let Some(x) = filter.status.clone() {
        x
            .split(",")
            .map(|x| x.into())
            .collect::<Vec<_>>()
    } else {
        vec![
            "DRAFT".into(),
            "READY_TO_START".into(),
            "IN_PROGRESS".into(),
            "PAUSED".into(),
            "DONE".into(),
        ]
    };

    let entries = sqlx::query!(r#"
            SELECT
                id,
                name,
                status AS "status: ProjectStatus",
                orderer,
                sell_price,
                project_group_id
            FROM project
            WHERE
                (
                    NOT (LOWER(name) LIKE '%' || LOWER($2) || '%') IS FALSE AND
                    NOT (status = ANY($3::PROJECT_STATUS[])) IS FALSE AND
                    NOT (LOWER(orderer) LIKE '%' || LOWER($4) || '%') IS FALSE AND
                    NOT (project_group_id = $5::UUID) IS FALSE
                )
                AND
                (
                    -- check if the character is in the group
                    (
                        NOT (project_group_id = ANY($6::UUID[])) IS FALSE
                    )
                    OR
                    -- as a fallback check if the character is the owner
                    (
                        owner = $1
                    )
                )
            ORDER BY name
        "#,
            *character_id,
            filter.name,
            &filter_status as _,
            filter.orderer,
            filter.project_group_id.map(|x| *x),
            &user_project_groups,
        )
        .fetch_all(pool)
        .await
        .map_err(ProjectError::List)?;

    let project_groups = crate::project_group::service::list(
            pool,
            character_id,
            ProjectGroupFilter::default(),
        )
        .await?
        .into_iter()
        .map(|x| (x.id, x))
        .collect::<HashMap<_, _>>();

    let mut projects = Vec::new();
    for entry in entries {
        let project_group = if let Some(x) = project_groups.get(&entry.project_group_id.into()) {
            x.clone()
        } else {
            continue;
        };

        let project_group = ProjectMinimal {
            id:            entry.id.into(),
            name:          entry.name,
            status:        entry.status,
            orderer:       entry.orderer,
            sell_price:    entry.sell_price,
            project_group: project_group,
        };
        projects.push(project_group);
    }

    Ok(projects)
}

#[cfg(test)]
mod list_project_group_test {
    use sqlx::PgPool;
    use starfoundry_lib_types::CharacterId;

    use crate::project::service::list::ProjectFilter;

    #[sqlx::test(
        fixtures(
            path = "../fixtures",
            scripts("base")
        ),
    )]
    async fn happy_path(
        pool: PgPool,
    ) {
        let result = super::list(
                &pool,
                CharacterId(1),
                ProjectFilter::default(),
            )
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 4);

        let result = super::list(
                &pool,
                CharacterId(1),
                ProjectFilter {
                    name: Some(String::from("Filter")),
                    ..Default::default()
                },
            )
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 1);

        let result = super::list(
                &pool,
                CharacterId(1),
                ProjectFilter {
                    name: Some(String::from("SomeGibberish")),
                    ..Default::default()
                },
            )
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }
}
