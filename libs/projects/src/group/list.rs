use sqlx::PgPool;
use starfoundry_lib_types::CharacterId;

use crate::{Error, ProjectGroup, Result};
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};

#[deprecated]
pub async fn list(
    pool:         &PgPool,
    character_id: CharacterId,
    filter:       ProjectGroupFilter,
) -> Result<Vec<ProjectGroup>> {
    let owner_filter = match filter.owner {
        Some(true)  => false,
        Some(false) |
        None        => true,
    };

    sqlx::query!(
        "
            SELECT
                pg.id,
                pg.name,
                pg.description,
                pg.owner = $1 AS is_owner,
                (
                    SELECT COUNT(*)
                    FROM project_group_member
                    WHERE group_id = pg.id
                ) AS members,
                (
                    SELECT COUNT(*)
                    FROM project
                    WHERE project_group_id = pg.id
                ) AS projects
            FROM project_group pg
            JOIN project_group_member pgm ON pg.id = pgm.group_id
            -- fetch all projects where the user is a member
            WHERE pgm.character_id = $1 AND
                NOT (LOWER(name) LIKE '%' || LOWER($2) || '%') IS FALSE AND
                NOT (owner = $1 OR $3) IS FALSE AND
                -- make sure the user got accepted into the group
                accepted = TRUE
            ORDER BY pg.name ASC
        ",
            *character_id,
            filter.name,
            owner_filter,
        )
        .fetch_all(pool)
        .await
        .map(|entries| {
            entries
                .into_iter()
                .map(|x| ProjectGroup {
                    id: x.id,
                    name: x.name,
                    members: x.members.unwrap_or(1),
                    projects: x.projects.unwrap_or(0),
                    is_owner: x.is_owner.unwrap_or_default(),

                    description: x.description,
                })
                .collect::<Vec<_>>()
        })
        .map_err(Error::ListGroups)
}

#[derive(Debug, Default, Deserialize, ToSchema, IntoParams)]
#[into_params(parameter_in = Query)]
#[deprecated]
pub struct ProjectGroupFilter {
    #[serde(default)]
    #[param(
        example = json!("ProjectGroup1337"),
        required = false,
    )]
    pub name:  Option<String>,
    #[serde(default)]
    pub owner: Option<bool>,
}

#[cfg(test)]
mod list_project_group_test {
    use sqlx::PgPool;
    use starfoundry_lib_types::CharacterId;

    use crate::ProjectGroupFilter;

    #[sqlx::test(
        fixtures("list"),
        migrator = "crate::test_util::MIGRATOR"
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
                }
            )
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 4);
    }
}
