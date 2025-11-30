use serde::Deserialize;
use sqlx::PgPool;
use starfoundry_lib_eve_gateway::{EveGatewayApiClient, StructurePosition};
use starfoundry_lib_types::CharacterId;
use std::collections::HashMap;
use std::fmt;
use utoipa::IntoParams;

use crate::structure::error::Result;
use crate::structure::service::Structure;
use crate::structure::{StructureError, StructureUuid};

pub async fn list(
    pool:                   &PgPool,
    eve_gateway_api_client: &impl EveGatewayApiClient,
    character_id:           CharacterId,
    filter:                 StructureFilter,
) -> Result<Vec<Structure>> {
    let structures = sqlx::query!(r#"
            SELECT
                id,
                type_id,
                structure_id,
                name AS "structure_name",
                services,
                rigs,
                system_id,
                x,
                y,
                z
            FROM structure
            WHERE
                NOT (LOWER(structure.name) LIKE '%' || LOWER($2) || '%') IS FALSE AND
                NOT (structure.type_id = $3) IS FALSE AND
                NOT (structure.system_id = $4) IS FALSE AND
                NOT ($5::INTEGER IS NULL OR $5::INTEGER = ANY(services)) IS FALSE AND
                NOT ($6::INTEGER IS NULL OR $6::INTEGER = ANY(rigs)) IS FALSE AND
                NOT ($7::UUID[] IS NULL OR id = ANY($7)) IS FALSE AND
                (owner = $1 OR owner = 0) -- owner = 0 is for NPC stations
            ORDER BY structure.name
        "#,
            *character_id,
            filter.name,
            filter.structure_type_id,
            filter.system_id,
            filter.service_id,
            filter.rig_id,
            &filter.structure_ids as _,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| StructureError::ListStructures(e))?
        .into_iter()
        .filter(|x| {
            if let Some(true) = filter.include_npc {
                true
            } else {
                if x.type_id == 46767 || x.type_id == 52678 {
                    false
                } else {
                    true
                }
            }
        })
        .collect::<Vec<_>>();

    let mut type_ids = structures
        .iter()
        .map(|x| x.type_id)
        .map(Into::into)
        .collect::<Vec<_>>();
    let service_type_ids = structures
        .iter()
        .flat_map(|x| x.services.clone())
        .map(Into::into)
        .collect::<Vec<_>>();
    type_ids.extend_from_slice(&service_type_ids);
    type_ids.sort();
    type_ids.dedup();

    let system_ids = structures
        .iter()
        .map(|x| x.system_id)
        .map(Into::into)
        .collect::<Vec<_>>();
    type_ids.sort();
    type_ids.dedup();

    let resolved_type_ids = eve_gateway_api_client
        .fetch_item_bulk(type_ids)
        .await?
        .into_iter()
        .map(|x| (x.type_id, x))
        .collect::<HashMap<_, _>>();
    let resolved_system_ids = eve_gateway_api_client
        .fetch_system_bulk(system_ids)
        .await?
        .into_iter()
        .map(|x| (x.system_id, x))
        .collect::<HashMap<_, _>>();

    let mut structure_result = Vec::new();
    for structure in structures {
        // TODO: add bulk function
        let mut rigs = Vec::new();
        for rig in structure.rigs {
            if let Ok(Some(x)) = eve_gateway_api_client.fetch_rig(rig.into()).await {
                rigs.push(x);
            } else {
                // silently ignore services that weren't found
                tracing::debug!("Couldn't find rig {}", rig);
                continue;
            }
        }

        let mut services = Vec::new();
        for service in structure.services {
            if let Some(x) = resolved_type_ids.get(&service.into()) {
                services.push(x.clone());
            } else {
                // silently ignore services that weren't found
                tracing::debug!("Couldn't find service {}", service);
                continue;
            }
        }

        let structure_item = if let Some(x) = resolved_type_ids.get(&structure.type_id.into()) {
            x
        } else {
            // silently ignore type_ids that weren't found
            tracing::debug!("Couldn't find type_id {}", structure.type_id);
            continue;
        };

        let system = if let Some(x) = resolved_system_ids.get(&structure.system_id.into()) {
            x
        } else {
            // silently ignore system that weren't found
            tracing::debug!("Couldn't find system {}", structure.system_id);
            continue;
        };

        let structure = Structure {
            id:                     structure.id.into(),
            name:                   structure.structure_name,
            structure_id:           structure.structure_id,
            system:                 system.clone(),
            item:                   structure_item.clone(),
            rigs:                   rigs,
            services:               services,
            position:               StructurePosition {
                                        x: structure.x,
                                        y: structure.y,
                                        z: structure.z
                                    },

            installable_rigs:       None,
            installable_services:   None,
        };
        structure_result.push(structure);
    }

    Ok(structure_result)
}

#[cfg(test)]
mod list_structure_test {
    use sqlx::PgPool;
    use starfoundry_lib_types::CharacterId;

    use crate::test_util::EveGatewayTestApiClient;
    use crate::structure::service::StructureFilter;

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
                StructureFilter::default(),
            )
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 2);

        let result = super::list(
                &pool,
                &gateway_client,
                CharacterId(1),
                StructureFilter {
                    name:  Some(String::from("Filter")),
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
                StructureFilter {
                    name: Some(String::from("SomeGibberish")),
                    ..Default::default()
                }
            )
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);

        let result = super::list(
                &pool,
                &gateway_client,
                CharacterId(2),
                StructureFilter {
                    service_id: Some(35892),
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
                StructureFilter {
                    rig_id: Some(46497),
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
                StructureFilter {
                    structure_type_id: Some(35892),
                    ..Default::default()
                }
            )
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 2);
    }
}

#[derive(Debug, Default, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct StructureFilter {
    #[serde(default)]
    #[param(
        example = json!("My cool structure"),
        required = false,
    )]
    pub name:              Option<String>,

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

    /// List of StructureUuids that should be fetched
    #[serde(default)]
    #[param(
        example = json!(["019a4147-4f81-7478-9598-71223d284470"]),
        required = false,
    )]
    pub structure_ids:    Option<Vec<StructureUuid>>,

    /// Includes NPC stations
    #[serde(default)]
    #[param(
        example = json!(true),
        required = false,
    )]
    pub include_npc:      Option<bool>,
}

impl fmt::Display for StructureFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self, f)
    }
}
