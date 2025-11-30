use sqlx::PgPool;
use std::collections::HashMap;
use std::str::FromStr;
use uuid::Uuid;

use crate::Mapping;

pub async fn migrate_project_group(
    postgres_source:      &PgPool,
    postgres_destination: &PgPool,
    structure_mapping:    &Mapping,
) -> Result<Mapping, Box<dyn std::error::Error>> {
    dbg!("Start - project group");
    let mut mappings = HashMap::new();

    let project_groups = sqlx::query!(r#"
            SELECT
                id,
                name,
                owner,
                description,
                created_at,
                updated_at
            FROM project_group
        "#)
        .fetch_all(postgres_source)
        .await?;

    let mut transaction = postgres_destination.begin().await?;
    for project_group in project_groups {
        if project_group.id == Uuid::default() {
            continue;
        } else if project_group.id == Uuid::from_str("dda037ce-9f5c-4a0f-8767-db5b38575b91").unwrap() {
            continue;
        } else if project_group.id == Uuid::from_str("019ab766-e3b4-79e1-943a-2ec3dcaecc4d").unwrap() {
            continue;
        }

        let project_group_id = Uuid::now_v7();
        mappings.insert(project_group.id, project_group_id);

        sqlx::query!("
                INSERT INTO project_group
                (
                    id,
                    name,
                    owner,
                    description,
                    created_at,
                    updated_at
                )
                VALUES ($1, $2, $3, $4, $5, $6)
            ",
                project_group_id,
                project_group.name,
                project_group.owner,
                project_group.description,
                project_group.created_at,
                project_group.updated_at,
            )
            .execute(&mut *transaction)
            .await?;

        let blacklist = sqlx::query!("
                SELECT
                    project_group_id,
                    type_id,
                    created_at,
                    updated_at
                FROM project_group_default_blacklist
                WHERE project_group_id = $1
            ",
                project_group.id
            )
            .fetch_all(postgres_source)
            .await?;
        sqlx::query!("
                INSERT INTO project_group_default_blacklist
                (
                    project_group_id,
                    type_id,
                    created_at,
                    updated_at
                )
                SELECT $1, * FROM UNNEST(
                    $2::INTEGER[],
                    $3::TIMESTAMPTZ[],
                    $4::TIMESTAMPTZ[]
                )
            ",
                project_group_id,
                &blacklist.iter().map(|x| x.type_id).collect::<Vec<_>>(),
                &blacklist.iter().map(|x| x.created_at).collect::<Vec<_>>(),
                &blacklist.iter().map(|x| x.updated_at).collect::<Vec<_>>(),
            )
            .execute(&mut *transaction)
            .await?;

        let market = sqlx::query!("
                SELECT
                    project_group_id,
                    structure_id,
                    created_at,
                    updated_at
                FROM project_group_default_market
                WHERE project_group_id = $1
            ",
                project_group.id
            )
            .fetch_all(postgres_source)
            .await?;
        sqlx::query!("
                INSERT INTO project_group_default_market
                (
                    project_group_id,
                    structure_id,
                    created_at,
                    updated_at
                )
                SELECT $1, * FROM UNNEST(
                    $2::UUID[],
                    $3::TIMESTAMPTZ[],
                    $4::TIMESTAMPTZ[]
                )
            ",
                project_group_id,
                &market
                    .iter()
                    .map(|x| x.structure_id)
                    .map(|x| structure_mapping.get(&x).unwrap().clone())
                    .collect::<Vec<_>>(),
                &market.iter().map(|x| x.created_at).collect::<Vec<_>>(),
                &market.iter().map(|x| x.updated_at).collect::<Vec<_>>(),
            )
            .execute(&mut *transaction)
            .await?;

        let member = sqlx::query!("
                SELECT
                    group_id,
                    character_id,
                    accepted,
                    created_at,
                    updated_at
                FROM project_group_member
                WHERE group_id = $1
            ",
                project_group.id
            )
            .fetch_all(postgres_source)
            .await?;
        sqlx::query!("
                INSERT INTO project_group_member
                (
                    project_group_id,
                    character_id,
                    accepted,
                    permission,
                    created_at,
                    updated_at
                )
                SELECT $1, * FROM UNNEST(
                    $2::INTEGER[],
                    $3::BOOLEAN[],
                    $4::INTEGER[],
                    $5::TIMESTAMPTZ[],
                    $6::TIMESTAMPTZ[]
                )
            ",
                project_group_id,
                &member
                    .iter()
                    .map(|x| x.character_id)
                    .collect::<Vec<_>>(),
                &member
                    .iter()
                    .map(|x| x.accepted)
                    .collect::<Vec<_>>(),
                &member
                    .iter()
                    .map(|x| {
                        return if project_group.owner == x.character_id {
                            1
                        } else {
                            2
                        }
                    })
                    .collect::<Vec<_>>(),
                &member.iter().map(|x| x.created_at).collect::<Vec<_>>(),
                &member.iter().map(|x| x.updated_at).collect::<Vec<_>>(),
            )
            .execute(&mut *transaction)
            .await?;
    }
    transaction.commit().await?;
    dbg!("Done - project group");

    Ok(mappings)
}
