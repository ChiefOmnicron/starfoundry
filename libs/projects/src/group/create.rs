use sqlx::PgPool;
use starfoundry_lib_types::CharacterId;
use std::str::FromStr;
use uuid::Uuid;

use crate::{CreateProjectGroup, Error, ProjectGroupPermissionCode, ProjectGroupUuid, Result};

#[deprecated]
pub async fn create(
    pool:         &PgPool,
    character_id: CharacterId,
    info:         CreateProjectGroup,
) -> Result<ProjectGroupUuid> {
    info.valid()?;

    let mut transaction = pool
        .begin()
        .await
        .map_err(Error::TransactionBeginError)?;

    // create the group
    let group_id = sqlx::query!("
            INSERT INTO project_group(
                owner,
                name,
                description
            )
            VALUES ($1, $2, $3)
            RETURNING id
        ",
            *character_id,
            info.name,
            info.description,
        )
        .fetch_one(&mut *transaction)
        .await
        .map(|x| ProjectGroupUuid::new(x.id))
        .map_err(|e| Error::CreateGroup(e))?;

    // add the owner as member of the group
    sqlx::query!("
            INSERT INTO project_group_member(
                accepted,
                group_id,
                character_id
                --permission
            )
            VALUES (
                TRUE, $1, $2--, $3
            )
        ",
            *group_id,
            *character_id,
            //*ProjectGroupPermissionCode::Owner,
        )
        .execute(&mut *transaction)
        .await
        .map_err(|e| Error::AcceptGroupMember(e, group_id))?;

    // add the defaults
    sqlx::query!("
            INSERT INTO project_group_default_market(
                project_group_id,
                structure_id
            )
            VALUES (
                $1, $2
            )
        ",
            *group_id,
            Uuid::from_str("00000000-0000-0000-0000-000000000001").unwrap_or_default(), // jita
        )
        .execute(&mut *transaction)
        .await
        .map_err(|e| Error::AcceptGroupMember(e, group_id))?;

    transaction
        .commit()
        .await
        .map_err(Error::TransactionCommitError)?;
    Ok(group_id)
}

#[cfg(test)]
mod create_project_group_test {
    use sqlx::PgPool;
    use starfoundry_lib_types::CharacterId;

    use crate::{CreateProjectGroup, Error};

    #[sqlx::test]
    //#[sqlx::test(migrator = "crate::test_util::MIGRATOR")]
    async fn no_body(
        pool: PgPool,
    ) {
        let result = super::create(
                &pool,
                CharacterId(1),
                CreateProjectGroup {
                    name:        String::new(),
                    description: None,
                }
            )
            .await;
        assert!(result.is_err());
        assert!(matches!(result, Err(Error::ValidationError(_))));
    }

    #[sqlx::test]
    //#[sqlx::test(migrator = "crate::test_util::MIGRATOR")]
    async fn missing_name(
        pool: PgPool,
    ) {
        let result = super::create(
                &pool,
                CharacterId(1),
                CreateProjectGroup {
                    name:        String::new(),
                    description: Some(String::from("Test description")),
                }
            )
            .await;
        assert!(result.is_err());
        assert!(matches!(result, Err(Error::ValidationError(_))));
    }

    #[sqlx::test]
    //#[sqlx::test(migrator = "crate::test_util::MIGRATOR")]
    async fn happy_path(
        pool: PgPool,
    ) {
        let result = super::create(
                &pool,
                CharacterId(1),
                CreateProjectGroup {
                    name:        String::from("My shared projects"),
                    description: Some(String::from("My cool description")),
                }
            )
            .await;
        assert!(result.is_ok());

        let entry = sqlx::query!(
                "SELECT * FROM project_group WHERE id = $1",
                *result.unwrap(),
            )
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(entry.name, "My shared projects");
        assert_eq!(entry.description.unwrap(), "My cool description");
    }
}
