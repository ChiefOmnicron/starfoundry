use sqlx::PgPool;
use starfoundry_libs_types::CharacterId;
use std::str::FromStr;
use uuid::Uuid;

use crate::{CreateProjectGroup, Error, ProjectGroupUuid, Result};

pub async fn create(
    pool:         &PgPool,
    character_id: CharacterId,
    info:         CreateProjectGroup,
) -> Result<ProjectGroupUuid> {
    let mut transaction = pool
        .begin()
        .await
        .map_err(Error::TransactionBeginError)?;

    // create the group
    let group_id = sqlx::query!("
            INSERT INTO project_groups (
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
            INSERT INTO project_group_members (
                group_id,
                character_id,
                accepted,
                projects,
                project_group,
                structures
            )
            VALUES (
                $1, $2,
                TRUE,
                'WRITE',
                'WRITE',
                'WRITE'
            )
        ",
            *group_id,
            *character_id
        )
        .execute(&mut *transaction)
        .await
        .map_err(|e| Error::AcceptGroupMember(e, group_id))?;

    // add the defaults
    sqlx::query!("
            INSERT INTO project_group_default_markets(
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
