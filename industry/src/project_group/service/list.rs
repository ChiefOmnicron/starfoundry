use serde::Deserialize;
use sqlx::PgPool;
use starfoundry_lib_industry::project_group::ProjectGroupMinimal;
use starfoundry_lib_types::CharacterId;
use utoipa::{IntoParams, ToSchema};

use crate::project_group::error::{ProjectGroupError, Result};

pub async fn list(
    pool:           &PgPool,
    character_id:   CharacterId,
    filter:         ProjectGroupFilter,
) -> Result<Vec<ProjectGroupMinimal>> {
    let owner_filter = match filter.owner {
        Some(true)  => false,
        Some(false) |
        None        => true,
    };
    let archived_filter = match filter.archived {
        Some(true)  => true,
        Some(false) |
        None        => false,
    };

    let entries = sqlx::query!("
            SELECT
                pg.id,
                pg.name,
                pg.description,
                pg.owner = $1 AS is_owner,
                pg.archived,
                (
                    SELECT COUNT(*)
                    FROM project
                    WHERE project_group_id = pg.id
                ) AS projects
            FROM project_group pg
            JOIN project_group_member pgm ON pg.id = pgm.project_group_id
            -- fetch all projects where the user is a member
            WHERE pgm.character_id = $1 AND
                NOT (LOWER(name) LIKE '%' || LOWER($2) || '%') IS FALSE AND
                NOT (owner = $1 OR $3) IS FALSE AND
                archived = $4
            ORDER BY pg.name ASC
        ",
            *character_id,
            filter.name,
            owner_filter,
            archived_filter,
        )
        .fetch_all(pool)
        .await
        .map_err(ProjectGroupError::ListGroups)?;

    let mut project_groups = Vec::new();
    for entry in entries {
        let project_group = ProjectGroupMinimal {
            id:             entry.id.into(),
            name:           entry.name,
            project_count:  entry.projects.unwrap_or(0),
            is_owner:       entry.is_owner.unwrap_or_default(),
            description:    entry.description,
            archived:       entry.archived,
        };
        project_groups.push(project_group);
    }

    Ok(project_groups)
}

#[derive(Debug, Default, Deserialize, ToSchema, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct ProjectGroupFilter {
    #[serde(default)]
    #[param(
        example = json!("ProjectGroup1337"),
        required = false,
    )]
    pub name:  Option<String>,

    #[serde(default)]
    pub owner: Option<bool>,

    #[serde(default)]
    pub archived: Option<bool>,
}

#[cfg(test)]
mod list_project_group_test {
    use sqlx::PgPool;
    use starfoundry_lib_types::CharacterId;

    use crate::project_group::service::list::ProjectGroupFilter;

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
                ProjectGroupFilter {
                    name: None,
                    owner: None,
                    archived: Some(false),
                }
            )
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 4);

        let result = super::list(
                &pool,
                CharacterId(1),
                ProjectGroupFilter {
                    name: Some(String::from("Filter")),
                    owner: None,
                    archived: Some(false),
                }
            )
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 2);

        let result = super::list(
                &pool,
                CharacterId(1),
                ProjectGroupFilter {
                    name: Some(String::from("SomeGibberish")),
                    owner: None,
                    archived: Some(false),
                }
            )
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);

        let result = super::list(
                &pool,
                CharacterId(2),
                ProjectGroupFilter {
                    name: None,
                    owner: Some(true),
                    archived: Some(false),
                }
            )
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 1);

        let result = super::list(
                &pool,
                CharacterId(1),
                ProjectGroupFilter {
                    name: None,
                    owner: Some(false),
                    archived: Some(false),
                }
            )
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 4);
    }
}
