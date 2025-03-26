//! Creates the SQL-Code for blueprints
use sqlx::PgPool;
use starfoundry_libs_types::{GroupId, TypeId};
use std::collections::HashMap;
use std::time::Instant;

use crate::Error;
use crate::parser::type_dogma::TypeDogma;
use crate::parser::type_ids::TypeIdEntry;

pub async fn run(
    pool:       &PgPool,
    type_ids:   &HashMap<TypeId, TypeIdEntry>,
    type_dogma: &HashMap<usize, TypeDogma>,
) -> Result<(), Error> {
    tracing::info!("Processing structure rigs");
    let start = Instant::now();

    let structure_rigs = prepare_data(
        &type_ids,
        &type_dogma
    ).await?;

    let mut transaction = pool
        .begin()
        .await
        .map_err(Error::TransactionError)?;

    tracing::debug!("Clearing database");
    sqlx::query!("
            DELETE FROM structure_rigs
        ")
        .execute(&mut *transaction)
        .await
        .map_err(Error::DeleteItems)?;
    tracing::debug!("Clearing database done");

    tracing::debug!("Inserting data starting");
    for structure in structure_rigs {
        sqlx::query!("
            INSERT INTO structure_rigs
            (
                type_id,
                structures
            )
            VALUES (
                $1, $2
            )
        ",
            &*structure.type_id,
            &structure.structure_type_ids.into_iter().map(|x| *x).collect::<Vec<_>>() as _,
        )
        .execute(&mut *transaction)
        .await
        .unwrap();
    }
    tracing::debug!("Inserting data done");

    transaction
        .commit()
        .await
        .map_err(Error::TransactionError)?;
    tracing::debug!("Transaction commited");

    tracing::info!(
        "Finished processing items, task took {:.2}s",
        start.elapsed().as_secs_f64()
    );

    Ok(())
}

async fn prepare_data(
    type_ids:   &HashMap<TypeId, TypeIdEntry>,
    type_dogma: &HashMap<usize, TypeDogma>,
) -> Result<Vec<Structure>, Error> {
    let market_groups = vec![
        // Medium Structure Engineering Rigs
        GroupId(2347),
        // Large Structure Engineering Rigs
        GroupId(2348),
        // X-Large Structure Engineering Rigs
        GroupId(2349),
        
        // Medium Structure Resource Processing Rigs
        GroupId(2341),
        // Large Structure Resource Processing Rigs
        GroupId(2342),
        // X-Large Structure Resource Processing Rigs
        GroupId(2343),
    ];

    let mut structures = Vec::new();

    for (type_id, entry) in type_ids {
        if let Some(x) = entry.market_group_id {
            if !market_groups.contains(&x) {
                continue;
            }
        } else {
            continue;
        }

        let name = if let Some(x) = entry.name() {
            if x.contains("OLD") {
                continue;
            }
            x
        } else {
            continue;
        };

        let dogma = if let Some(x) = type_dogma.get(&(**type_id as usize)) {
            x
        } else {
            continue;
        };

        let mut structure_type_ids = Vec::new();

        let attribute_ids = vec![
            1298,
            1299,
            1300,
        ];

        for attribute in dogma.attributes.iter() {
            if attribute_ids.contains(&attribute.attribute_id) {
                if vec![1404, 1406, 1657].contains(&(attribute.value as u32)) {
                    structure_type_ids.extend(group_to_ids(attribute.value as u32, &name));
                } else {
                    structure_type_ids.push((attribute.value as u32).into());
                }
            }
        }

        let type_id = *type_id;

        let item = Structure {
            type_id,
            structure_type_ids,
        };
        structures.push(item);
    }

    Ok(structures)
}

fn group_to_ids(group: u32, name: &str) -> Vec<TypeId> {
    let size = if name.starts_with("Standup M-Set") {
        "Standup M-Set"
    } else if name.starts_with("Standup L-Set") {
        "Standup L-Set"
    } else if name.starts_with("Standup XL-Set") {
        "Standup XL-Set"
    } else {
        ""
    };

    match (group, size) {
        (1404, "Standup M-Set") => vec![
            // Raitaru
            TypeId(35825),
        ],
        (1404, "Standup L-Set") => vec![
            // Azbel
            TypeId(35826),
        ],
        (1404, "Standup XL-Set") => vec![
            // Sotiyo
            TypeId(35827),
        ],
        (1406, "Standup M-Set") => vec![
            // Athanor
            TypeId(35835),
        ],
        (1406, "Standup L-Set") => vec![
            // Tatara
            TypeId(35836),
        ],
        (1657, "Standup M-Set") => vec![
            // Astrahus
            TypeId(35832),
        ],
        (1657, "Standup L-Set") => vec![
            // Fortizar
            TypeId(35833),
            // 'Moreau' Fortizar
            TypeId(47512),
            // 'Draccous' Fortizar
            TypeId(47513),
            // 'Horizon' Fortizar
            TypeId(47514),
            // 'Marginis' Fortizar
            TypeId(47515),
            // 'Prometheus' Fortizar
            TypeId(47516),
        ],
        (1657, "Standup XL-Set") => vec![
            // Keepstar
            TypeId(35834),
            // Upwell Palatine Keepstar
            TypeId(40340),
        ],
        _ => Vec::new(),
    }
}

#[derive(Debug)]
struct Structure {
    type_id:            TypeId,
    structure_type_ids: Vec<TypeId>,
}
