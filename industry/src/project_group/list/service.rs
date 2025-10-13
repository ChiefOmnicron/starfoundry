use sqlx::PgPool;
use starfoundry_lib_eve_gateway::ApiClient;
use starfoundry_lib_types::CharacterId;

use crate::project_group::error::{ProjectGroupError, Result};
use crate::project_group::list::{ProjectGroup, ProjectGroupFilter};
use crate::project_group::list_members::list_members;
use crate::project_group::list_default_blacklist::list_default_blacklist;
use crate::project_group::list_default_market::list_default_market;

pub async fn list(
    pool:           &PgPool,
    gateway_client: &impl ApiClient,
    character_id:   CharacterId,
    filter:         ProjectGroupFilter,
) -> Result<Vec<ProjectGroup>> {
    let owner_filter = match filter.owner {
        Some(true)  => false,
        Some(false) |
        None        => true,
    };

    let entries = sqlx::query!(
        "
            SELECT
                pg.id,
                pg.name,
                pg.description,
                pg.owner = $1 AS is_owner,
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
        .map_err(ProjectGroupError::ListGroups)?;

    let mut project_groups = Vec::new();
    for entry in entries {
        let project_group = ProjectGroup {
            id:             entry.id.into(),
            name:           entry.name,
            project_count:  entry.projects.unwrap_or(0),
            is_owner:       entry.is_owner.unwrap_or_default(),
            description:    entry.description,

            default_blacklist: list_default_blacklist(pool, entry.id.into()).await?,
            default_market:    list_default_market(pool, entry.id.into()).await?,
            members:           list_members(
                pool,
                gateway_client,
                entry.id.into()
            ).await?,
        };
        project_groups.push(project_group);
    }

    Ok(project_groups)
}

#[cfg(test)]
mod list_project_group_test {
    use sqlx::PgPool;
    use starfoundry_lib_types::CharacterId;

    use crate::project_group::list::filter::ProjectGroupFilter;
    use crate::test::TestApiClient;

    #[sqlx::test(
        fixtures(
            path = "../fixtures",
            scripts("base")
        ),
        migrator = "crate::test_util::MIGRATOR"
    )]
    async fn happy_path(
        pool: PgPool,
    ) {
        let gateway_client = TestApiClient::new();
        let result = super::list(
                &pool,
                &gateway_client,
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
                &gateway_client,
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
                &gateway_client,
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
                &gateway_client,
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
                &gateway_client,
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
