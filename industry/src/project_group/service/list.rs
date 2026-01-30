use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use starfoundry_lib_eve_gateway::EveGatewayApiClient;
use starfoundry_lib_types::CharacterId;
use utoipa::{IntoParams, ToSchema};

use crate::project_group::error::{ProjectGroupError, Result};
use crate::project_group::ProjectGroupUuid;
use crate::project_group::service::{ProjectGroupMember, list_members};

pub async fn list(
    pool:                   &PgPool,
    character_id:           CharacterId,
    eve_gateway_api_client: &impl EveGatewayApiClient,
    filter:                 ProjectGroupFilter,
) -> Result<Vec<ProjectGroup>> {
    let owner_filter = match filter.owner {
        Some(true)  => false,
        Some(false) |
        None        => true,
    };

    let entries = sqlx::query!("
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
            JOIN project_group_member pgm ON pg.id = pgm.project_group_id
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
            members:        list_members(
                pool,
                eve_gateway_api_client,
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

    use crate::project_group::service::list::ProjectGroupFilter;
    use crate::test_util::EveGatewayTestApiClient;

    #[sqlx::test(
        fixtures(
            path = "../fixtures",
            scripts("base")
        ),
    )]
    async fn happy_path(
        pool: PgPool,
    ) {
        let gateway_client = EveGatewayTestApiClient::new();
        let result = super::list(
                &pool,
                CharacterId(1),
                &gateway_client,
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
                &gateway_client,
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
                &gateway_client,
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
                &gateway_client,
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
                &gateway_client,
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
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
#[schema(
    example = json!({
        "id": "b034c3a9-2f4d-487d-95bb-c66fc20148b3",
        "name": "My cool group",
        "project_count": 100,
        "is_owner": true,
        "description": "Bunch of cool projects",
        "members": [{
            "character_name": "SomeCharacterName",
            "character_id": 1337,

            "accepted": true,
            "permission": [
                "READ",
                "WRITE_GROUP"
            ],
            "is_owner": false
        }]
    })
)]
pub struct ProjectGroup {
    pub id:            ProjectGroupUuid,
    pub name:          String,
    pub project_count: i64,
    pub is_owner:      bool,
    pub description:   Option<String>,
    pub members:       Vec<ProjectGroupMember>,
}
