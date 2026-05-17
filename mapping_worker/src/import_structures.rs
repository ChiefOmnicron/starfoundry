use starfoundry_lib_eve_gateway::{EveGatewayApiClient, EveGatewayApiClientSearch, EveGatewayClient, SearchCategory};
use starfoundry_lib_types::StructureId;
use std::collections::HashMap;

    let eve_gateway_client = EveGatewayClient::new(SERVICE_NAME)?;

    let systems = sqlx::query!("
            SELECT system_id, system_name
            FROM system
        ")
        .fetch_all(&pool)
        .await?
        .into_iter()
        .map(|x| (x.system_id, x.system_name))
        .collect::<HashMap<_, _>>();

    tracing::info!("Found {} systems", systems.len());

    let mut total_count_structures = 0usize;
    let mut structures_by_system = HashMap::new();

    for (index, (system_id, system_name)) in systems.iter().enumerate() {
        tracing::info!(
            "[{:4} / {:4}] Start search",
            index + 1,
            systems.len(),
        );

        let search = eve_gateway_client
            .in_game_search(
                2117441999.into(),
                "industry.dev.starfoundry.space".into(),
                SearchCategory::Structure,
                system_name,
            )
            .await?;
        let ids = search.0
            .into_iter()
            .map(Into::into)
            .collect::<Vec<_>>();
        total_count_structures += ids.len();

        tracing::info!(
            "Adds {} structures to {}",
            ids.len(),
            system_name,
        );

        structures_by_system
            .entry(system_id)
            .and_modify(|x: &mut Vec<StructureId>| x.extend(ids.clone()))
            .or_insert(ids);
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    tracing::info!("Found {} total structures", total_count_structures);

    for (system_index, (system_id, structures)) in structures_by_system.iter().enumerate() {
        let system_name = systems.get(system_id).cloned().unwrap_or(String::new());
        tracing::info!(
            "[{:3} / {:3}] Start system {} fetch",
            system_index + 1,
            structures_by_system.len(),
            system_name,
        );

        for (structure_index, structure_id) in structures.iter().enumerate() {
            tracing::info!(
                "[{:3} / {:3}] [{:3} / {:3}] Start structure",
                system_index + 1,
                structures_by_system.len(),
                structure_index + 1,
                structures.len(),
            );

            let result = eve_gateway_client
                .resolve_structure(
                    2117441999.into(),
                    "industry.dev.starfoundry.space".into(),
                    *structure_id,
                )
                .await?
                .unwrap();

            sqlx::query!("
                    INSERT INTO mapping_structure (
                        owner,
                        structure_id,
                        system_id,
                        type_id,
                        name,
                        x,
                        y,
                        z
                    )
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                    ON CONFLICT (structure_id) DO UPDATE
                        SET name = EXCLUDED.name
                ",
                    2117441999,
                    **structure_id,
                    system_id,
                    *result.item.type_id,
                    result.name,
                    result.position.x,
                    result.position.y,
                    result.position.z,
                )
                .execute(&pool)
                .await?;
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }
