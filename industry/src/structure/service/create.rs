use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use starfoundry_lib_eve_gateway::StructurePosition;
use starfoundry_lib_industry::StructureUuid;
use starfoundry_lib_types::{SystemId, TypeId};
use starfoundry_lib_types::CharacterId;
use utoipa::ToSchema;

use crate::structure::StructureError;
use crate::structure::error::Result;
use std::collections::HashMap;

pub async fn create(
    pool:         &PgPool,
    character_id: CharacterId,
    info:         CreateStructure,
) -> Result<StructureUuid> {
    info.valid()?;

    let mut transaction = pool
        .begin()
        .await
        .map_err(StructureError::BeginTransaction)?;

    let structure_id = sqlx::query!("
            INSERT INTO structure
            (
                owner,
                type_id,
                rigs,
                services,
                name,
                system_id,
                structure_id,
                x,
                y,
                z
            )
            VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING id
        ",
            *character_id,
            *info.structure_type_id,
            &info.rigs as _,
            &info.services as _,
            info.name,
            *info.system_id,
            info.structure_id,
            info.position.x,
            info.position.y,
            info.position.z,
        )
        .fetch_one(&mut *transaction)
        .await
        .map(|x| StructureUuid::new(x.id))
        .map_err(StructureError::CreateStructure)?;

    let mut type_ids = Vec::new();
    let mut taxes = Vec::new();
    info
        .taxes
        .into_iter()
        .for_each(|(type_id, tax)| {
            type_ids.push(*type_id);
            taxes.push(tax);
        });

    sqlx::query!("
            INSERT INTO structure_tax
            (
                structure_id,
                service_type_id,
                tax
            )
            SELECT $1, * FROM UNNEST(
                $2::INTEGER[],
                $3::REAL[]
            )
        ",
            *structure_id,
            &type_ids,
            &taxes,
        )
        .execute(&mut *transaction)
        .await
        .map_err(StructureError::CreateStructure)?;

    transaction
        .commit()
        .await
        .map_err(StructureError::CommitTransaction)?;

    Ok(structure_id)
}

#[cfg(test)]
mod create_project_group_test {
    use sqlx::PgPool;
    use starfoundry_lib_eve_gateway::{StructurePosition, StructureType};
    use starfoundry_lib_types::CharacterId;
    use std::collections::HashMap;

    use crate::structure::error::StructureError;
    use super::CreateStructure;

    #[sqlx::test()]
    async fn no_name(
        pool: PgPool,
    ) {
        let result = super::create(
                &pool,
                CharacterId(1),
                CreateStructure {
                    name:              String::new(),
                    system_id:         1337.into(),
                    structure_type_id: StructureType::Tatara.into(),
                    rigs:              Vec::new(),
                    services:          Vec::new(),
                    structure_id:      1_000_000_000_000,
                    position:          StructurePosition { x: 0f32, y: 0f32, z: 0f32 },
                    taxes:             HashMap::new(),
                }
            )
            .await;
        assert!(result.is_err());
        assert!(matches!(result, Err(StructureError::ValidationError(_))));
    }

    #[sqlx::test()]
    async fn structure_id_too_low(
        pool: PgPool,
    ) {
        let result = super::create(
                &pool,
                CharacterId(1),
                CreateStructure {
                    name:              String::from("My cool structure"),
                    system_id:         1337.into(),
                    structure_type_id: StructureType::Tatara.into(),
                    rigs:              Vec::new(),
                    services:          Vec::new(),
                    structure_id:      100_000_000_000,
                    position:          StructurePosition { x: 0f32, y: 0f32, z: 0f32 },
                    taxes:             HashMap::new(),
                }
            )
            .await;
        assert!(result.is_err());
        assert!(matches!(result, Err(StructureError::ValidationError(_))));
    }

    #[sqlx::test()]
    async fn happy_path(
        pool: PgPool,
    ) {
        let result = super::create(
                &pool,
                CharacterId(1),
                CreateStructure {
                    name:              String::from("My cool structure"),
                    system_id:         1337.into(),
                    structure_type_id: StructureType::Tatara.into(),
                    rigs:              Vec::new(),
                    services:          Vec::new(),
                    structure_id:      1_100_000_000_000,
                    position:          StructurePosition { x: 0f32, y: 0f32, z: 0f32 },
                    taxes:             HashMap::new(),
                }
            )
            .await;
        assert!(result.is_ok());

        let entry = sqlx::query!(r#"
                    SELECT
                        name,
                        type_id,
                        rigs,
                        services,
                        structure_id
                    FROM structure WHERE id = $1
                "#,
                *result.unwrap(),
            )
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(entry.name, "My cool structure");
        assert_eq!(entry.structure_id, 1_100_000_000_000);
    }
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(
    example = json!({
        "name": "1DQ1-A - 1-st Imperial Palace",
        "rigs": [
            37275
        ],
        "security": "NULLSEC",
        "services": [
            35894
        ],
        "structure_id": 1003520240,
        "structure_type_id": 35834,
        "system_id": 30004759
    })
)]
pub struct CreateStructure {
    /// Name of the structure
    pub name:              String,
    /// Location of the structure
    pub system_id:         SystemId,

    /// Type of structure
    pub structure_type_id: TypeId,
    /// List of all rigs that are in the structure
    pub rigs:              Vec<TypeId>,
    /// Id of the structure in-game
    pub services:          Vec<TypeId>,
    /// Position of the structure in the galaxy
    pub position:          StructurePosition,
    /// Taxes based on service type
    pub taxes:             HashMap<TypeId, f32>,

    /// EVE Id of the structure
    pub structure_id:      i64,
}

impl CreateStructure {
    pub fn valid(&self) -> Result<bool> {
        if self.name.len() <= 100 {
            if self.name.trim().is_empty() {
                return Err(StructureError::ValidationError("Field 'name' must be set".into()));
            }
        } else {
            return Err(StructureError::ValidationError("Field 'name' is too long, max length: 100".into()));
        };

        if self.structure_id < 1000000000000 {
            return Err(StructureError::ValidationError("Field 'structure_id' must be equal or larger than 1_000_000_000_000".into()));
        };

        Ok(true)
    }
}
