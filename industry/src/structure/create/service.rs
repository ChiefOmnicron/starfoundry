use sqlx::PgPool;
use starfoundry_lib_types::CharacterId;

use crate::structure::create::CreateStructure;
use crate::structure::{StructureError, StructureUuid};
use crate::structure::error::Result;

pub async fn create(
    pool:         &PgPool,
    character_id: CharacterId,
    info:         CreateStructure,
) -> Result<StructureUuid> {
    info.valid()?;

    sqlx::query!("
            INSERT INTO structure
            (
                owner,
                type_id,
                rigs,
                services,
                name,
                system_id,
                structure_id
            )
            VALUES($1, $2, $3, $4, $5, $6, $7)
            RETURNING id
        ",
            *character_id,
            *info.structure_type_id,
            &info.rigs as _,
            &info.services as _,
            info.name,
            *info.system_id,
            info.structure_id,
        )
        .fetch_one(pool)
        .await
        .map(|x| StructureUuid::new(x.id))
        .map_err(|e| StructureError::Create(e))
}


#[cfg(test)]
mod create_project_group_test {
    use sqlx::PgPool;
    use starfoundry_lib_eve_gateway::StructureType;
    use starfoundry_lib_types::CharacterId;

    use crate::structure::create::CreateStructure;
    use crate::structure::error::StructureError;

    #[sqlx::test(migrator = "crate::test_util::MIGRATOR")]
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
                }
            )
            .await;
        assert!(result.is_err());
        assert!(matches!(result, Err(StructureError::ValidationError(_))));
    }

    #[sqlx::test(migrator = "crate::test_util::MIGRATOR")]
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
                }
            )
            .await;
        assert!(result.is_err());
        assert!(matches!(result, Err(StructureError::ValidationError(_))));
    }

    #[sqlx::test(migrator = "crate::test_util::MIGRATOR")]
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
