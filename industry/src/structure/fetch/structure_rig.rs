use serde::Serialize;
use sqlx::PgPool;
use starfoundry_lib_eve_gateway::{EveGatewayApiClient, Item};
use starfoundry_lib_types::TypeId;
use utoipa::ToSchema;

use crate::structure::fetch::BonusModifier;
use crate::structure::error::{Result, StructureError};

#[derive(Debug, Serialize, ToSchema)]
#[cfg_attr(test, derive(serde::Deserialize))]
pub struct StructureRig {
    pub type_id:         TypeId,

    pub material:        Option<f32>,
    pub time:            Option<f32>,
    pub category_groups: Vec<i32>,

    pub item:            Item,
}

impl StructureRig {
    pub async fn new(
        pool:                   &PgPool,
        eve_gateway_api_client: &impl EveGatewayApiClient,
        type_id:                TypeId,
    ) -> Result<Option<Self>> {
        let mut material        = None;
        let mut time            = None;
        let mut category_groups = Vec::new();

        let item = if let Some(x) = eve_gateway_api_client.fetch_item(type_id).await? {
            x
        } else {
            return Ok(None);
        };

        let bonuses = sqlx::query!(r#"
                SELECT
                    modifier AS "modifier!: BonusModifier",
                    amount,
                    categories,
                    groups,
                    i.name
                FROM structure_dogma
                JOIN item i ON i.type_id = ptype_id
                WHERE ptype_id = $1
            "#,
                *type_id,
            )
            .fetch_all(pool)
            .await
            .map_err(|e| StructureError::FetchRigBonusByTypeId(e, type_id))?;

        for bonus in bonuses {
            match bonus.modifier {
                BonusModifier::ManufacturingMaterial |
                BonusModifier::ReactionMaterial    => {
                    material = Some(bonus.amount as f32);
                },
                BonusModifier::ManufactureTime |
                BonusModifier::ReactionTime    => {
                    time = Some(bonus.amount as f32);
                }
            }

            if category_groups.is_empty() {
                let mut cg = Vec::new();
                cg.extend(bonus.categories);
                cg.extend(bonus.groups);
                category_groups = cg;
            }
        }

        Ok(Some(StructureRig {
            type_id: type_id,
            material,
            time,
            category_groups,
            item,
        }))
    }
}
