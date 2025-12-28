use sqlx::PgPool;
use starfoundry_lib_eve_gateway::{EveGatewayApiClient, Item};
use starfoundry_lib_types::CharacterId;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::project_group::error::{ProjectGroupError, Result};
use crate::project_group::ProjectGroupUuid;
use crate::structure::service::Structure;
use crate::project_group::service::{ProjectGroupMember, list_default_blacklist, list_default_market, list_members};

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

            default_blacklist: list_default_blacklist(
                pool,
                eve_gateway_api_client,
                entry.id.into()
            ).await?,
            default_market:    list_default_market(
                pool,
                character_id,
                eve_gateway_api_client,
                entry.id.into()
            ).await?,
            members:           list_members(
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

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(
    example = json!({
        "id": "b034c3a9-2f4d-487d-95bb-c66fc20148b3",
        "name": "My cool group",
        "project_count": 100,
        "is_owner": true,
        "description": "Bunch of cool projects",
        "default_blacklist": [{
            "base_price": null,
            "category_id": 6,
            "group_id": 30,
            "meta_group_id": null,
            "name": "Ragnarok",
            "repackaged": 10000000,
            "type_id": 23773,
            "volume": 100000000
        }],
        "default_market": [{
            "id": "15bd47e3-6b38-4cc1-887b-94924fff30a1",
            "name": "1DQ1-A - RIP",
            "structure_id": 1337,
            "system": {
                "constellation_id": 20000696,
                "constellation_name": "O-EIMK",
                "region_id": 10000060,
                "region_name": "Delve",
                "system_id": 30004759,
                "system_name": "1DQ1-A",
                "security": -0.38578233,
                "security_group": "NULLSEC",
            },
            "structure_type": {
                "base_price": null,
                "category_id": 65,
                "group_id": 1657,
                "meta_group_id": 1,
                "name": "Keepstar",
                "repackaged": null,
                "type_id": 35834,
                "volume": 800000
            },
            "rigs": [],
            "service": [{
                "base_price": null,
                "category_id": 66,
                "group_id": 1321,
                "meta_group_id": 54,
                "name": "Standup Market Hub I",
                "repackaged": null,
                "type_id": 35892,
                "volume": 32000
            }]
        }],
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
    pub id:                ProjectGroupUuid,
    pub name:              String,
    pub project_count:     i64,
    pub is_owner:          bool,
    pub description:       Option<String>,

    pub default_blacklist: Vec<Item>,
    pub default_market:    Vec<Structure>,
    pub members:           Vec<ProjectGroupMember>,
}
