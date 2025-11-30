use serde::Deserialize;
use sqlx::PgPool;
use starfoundry_lib_eve_gateway::EveGatewayApiClient;
use starfoundry_lib_types::CharacterId;
use std::fmt;
use utoipa::IntoParams;

use crate::structure_group::error::{Result, StructureGroupError};
use crate::structure_group::service::StructureGroup;
use crate::structure::service::StructureFilter;

pub async fn list(
    pool:                   &PgPool,
    eve_gateway_api_client: &impl EveGatewayApiClient,
    character_id:           CharacterId,
    filter:                 StructureGroupFilter,
) -> Result<Vec<StructureGroup>> {
    let structure_groups = sqlx::query!(r#"
            SELECT
                DISTINCT(sg.id),
                sg.name
            FROM structure_group sg
            JOIN structure_group_structure sgs ON sgs.structure_group_id = sg.id
            JOIN structure s ON s.id = sgs.structure_id
            WHERE
                NOT (LOWER(sg.name) LIKE '%' || LOWER($2) || '%') IS FALSE AND
                NOT (s.type_id = $3) IS FALSE AND
                NOT (s.system_id = $4) IS FALSE AND
                NOT ($5::INTEGER IS NULL OR $5::INTEGER = ANY(s.services)) IS FALSE AND
                NOT ($6::INTEGER IS NULL OR $6::INTEGER = ANY(s.rigs)) IS FALSE AND
                sg.owner = $1
            ORDER BY sg.name
        "#,
            *character_id,
            filter.name,
            filter.structure_type_id,
            filter.system_id,
            filter.service_id,
            filter.rig_id,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| StructureGroupError::ListStructureGroups(e))?;

    let mut result = Vec::new();
    for structure_group in structure_groups {
        let structure_ids = sqlx::query!("
                SELECT
                    structure_id
                FROM structure_group_structure
                WHERE structure_group_id = $1
            ",
                structure_group.id,
            )
            .fetch_all(pool)
            .await
            .map_err(|e| StructureGroupError::FetchGroupStructures(e, structure_group.id.into()))?
            .into_iter()
            .map(|x| x.structure_id.into())
            .collect();

        let structures = crate::structure::service::list(
                &pool,
                eve_gateway_api_client,
                character_id,
                StructureFilter {
                    structure_ids: Some(structure_ids),
                    ..Default::default()
                }
            )
            .await?;

        result.push(StructureGroup {
            id:         structure_group.id.into(),
            name:       structure_group.name,
            structures: structures,
        });
    }

    Ok(result)
}

#[derive(Debug, Default, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct StructureGroupFilter {
    #[serde(default)]
    #[param(
        example = json!("My cool structure"),
        required = false,
    )]
    pub name: Option<String>,

    #[serde(default)]
    #[param(
        example = json!("35827"),
        required = false,
    )]
    pub structure_type_id: Option<i32>,

    #[serde(default)]
    #[param(
        example = json!("30004759"),
        required = false,
    )]
    pub system_id:         Option<i32>,

    /// [TypeId] of a structure service
    #[serde(default)]
    #[param(
        example = json!("35892"),
        required = false,
    )]
    pub service_id:        Option<i32>,

    /// [TypeId] of a structure rig
    #[serde(default)]
    #[param(
        example = json!("46497"),
        required = false,
    )]
    pub rig_id:           Option<i32>,
}

impl fmt::Display for StructureGroupFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self, f)
    }
}

#[cfg(test)]
mod list_structure_test {
    use sqlx::PgPool;
    use starfoundry_lib_types::CharacterId;

    use crate::test_util::EveGatewayTestApiClient;
    use crate::structure_group::service::StructureGroupFilter;

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
                &gateway_client,
                CharacterId(1),
                StructureGroupFilter::default(),
            )
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 1);

        let result = super::list(
                &pool,
                &gateway_client,
                CharacterId(1),
                StructureGroupFilter {
                    name:  Some(String::from("StructureGroupA")),
                    ..Default::default()
                }
            )
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 1);

        let result = super::list(
                &pool,
                &gateway_client,
                CharacterId(1),
                StructureGroupFilter {
                    name: Some(String::from("SomeGibberish")),
                    ..Default::default()
                }
            )
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }
}
