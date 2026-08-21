use starfoundry_lib_eve_gateway::{EveGatewayApiClient, EveGatewayApiClientSearch, EveGatewayApiClientSystem, EveGatewayClient};
use starfoundry_lib_gateway::Identity;
use std::collections::HashMap;
use sqlx::PgPool;

pub async fn populate_structure_database(
    pool: &PgPool,
) -> Result<(), Box<dyn std::error::Error>> {
    let identity = Identity::new(
        2117441999.into(),
        98024275.into(),
        "industry.dev.starfoundry.space".into(),
    );
    let eve_gateway_client = EveGatewayClient::new_with_identity("MAPPING_STARFOUNDRY", identity)?;

    let has_still_waiting = sqlx::query!("
            SELECT system_id
            FROM queue_system
            WHERE status = 'WAITING'
            LIMIT 1
        ")
        .fetch_optional(pool)
        .await?;

    // Fetch if there are no longer waiting tasks
    if has_still_waiting.is_none() {
        tracing::info!("Populating queue db");
        let systems = eve_gateway_client
            .list_systems()
            .await?;
        sqlx::query!("
                INSERT INTO queue_system(
                    system_id,
                    system_name
                )
                SELECT * FROM UNNEST(
                    $1::INTEGER[],
                    $2::VARCHAR[]
                )
            ",
                &systems.iter().map(|x| *x.system_id).collect::<Vec<_>>(),
                &systems.iter().map(|x| x.system_name.clone()).collect::<Vec<_>>(),
            )
            .execute(pool)
            .await?;
        tracing::info!("Populated queue db");
    }

    let systems = sqlx::query!("
            SELECT
                system_id,
                system_name
            FROM queue_system
            WHERE status = 'WAITING'
        ")
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|x| (x.system_id, x.system_name))
        .collect::<HashMap<_, _>>();

    tracing::info!("Found {} systems", systems.len());

    let mut total_count_structures = 0usize;

    for (index, (system_id, system_name)) in systems.iter().enumerate() {
        tracing::info!(
            "[{:4} / {:4}] Start search",
            index + 1,
            systems.len(),
        );

        let search = eve_gateway_client
            .search_structure(system_name)
            .await?;
        let ids = search
            .into_iter()
            .collect::<Vec<_>>();
        total_count_structures += ids.len();

        sqlx::query!("
                INSERT INTO queue_structure (
                    system_id,
                    structure_id
                )
                SELECT $1, * FROM UNNEST(
                    $2::BIGINT[]
                )
            ",
                system_id,
                &ids,
            )
            .execute(pool)
            .await?;
        sqlx::query!("
                UPDATE queue_system
                SET status = 'DONE'
                WHERE system_id = $1
            ",
                system_id,
            )
            .execute(pool)
            .await?;

        tracing::info!(
            "Adds {} structures to {}",
            ids.len(),
            system_name,
        );

        //std::thread::sleep(std::time::Duration::from_secs(1));
    }
    tracing::info!("Found {} total structures", total_count_structures);

    let structures = sqlx::query!("
            SELECT
                system_id,
                structure_id
            FROM queue_structure
            WHERE status = 'WAITING'
        ")
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|x| (x.system_id, x.structure_id))
        .collect::<Vec<_>>();

    let systems = sqlx::query!("
            SELECT
                system_id,
                system_name
            FROM queue_system
        ")
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|x| (x.system_id, x.system_name))
        .collect::<HashMap<_, _>>();

    for (system_index, (system_id, structure_id)) in structures.iter().enumerate() {
        let system_name = systems.get(system_id).cloned().unwrap_or(String::new());
        tracing::info!(
            "[{:3} / {:3}] Start system {} fetch",
            system_index + 1,
            structures.len(),
            system_name,
        );

        let result = match eve_gateway_client
            .resolve_structure(
                structure_id.into(),
            )
            .await {
                Ok(Some(x)) => x,
                Ok(None) => {
                    tracing::warn!("None");
                    sqlx::query!("
                            UPDATE queue_structure
                            SET status = 'WARN'
                            WHERE structure_id = $1
                        ",
                            structure_id,
                        )
                        .execute(pool)
                        .await?;
                    continue;
                },
                Err(e) => {
                    sqlx::query!("
                            UPDATE queue_structure
                            SET status = 'ERROR'
                            WHERE structure_id = $1
                        ",
                            structure_id,
                        )
                        .execute(pool)
                        .await?;
                    tracing::error!("{}", e.to_string());
                    continue;
                },
            };

        if !result.name.starts_with(&system_name) {
            continue;
        }

        sqlx::query!("
                INSERT INTO structure(
                    owner,
                    structure_id,
                    structure_owner,
                    system_id,
                    type_id,
                    name,
                    x,
                    y,
                    z
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
                ON CONFLICT (structure_id) DO UPDATE
                    SET name = EXCLUDED.name
            ",
                2117441999,
                *structure_id,
                *result.owner_id,
                *system_id,
                *result.item.type_id,
                result.name,
                result.position.x,
                result.position.y,
                result.position.z,
            )
            .execute(pool)
            .await?;

        sqlx::query!("
                UPDATE queue_structure
                SET status = 'DONE'
                WHERE structure_id = $1
            ",
                structure_id,
            )
            .execute(pool)
            .await?;
        //std::thread::sleep(std::time::Duration::from_secs(1));
    }

    Ok(())
}
