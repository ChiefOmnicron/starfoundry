//! Creates the SQL-Code for blueprints
use sqlx::PgPool;
use starfoundry_lib_types::{GroupId, TypeId};
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
            DELETE FROM structure_rig
        ")
        .execute(&mut *transaction)
        .await
        .map_err(Error::DeleteItems)?;
    sqlx::query!("
            DELETE FROM structure_service
        ")
        .execute(&mut *transaction)
        .await
        .map_err(Error::DeleteItems)?;
    tracing::debug!("Clearing database done");

    tracing::debug!("Inserting data starting");
    for structure in structure_rigs {
        sqlx::query!("
                INSERT INTO structure_rig
                (
                    type_id,
                    structures,
                    excludes
                )
                VALUES (
                    $1, $2, $3
                )
            ",
                &*structure.type_id,
                &structure.structure_type_ids.into_iter().map(|x| *x).collect::<Vec<_>>() as _,
                &structure.excluded.into_iter().map(|x| *x).collect::<Vec<_>>() as _,
            )
            .execute(&mut *transaction)
            .await
            .unwrap();
    }

    for structure in vec![35825, 35826, 35827, 35835, 35836, 35832, 35833, 47516, 47512, 47515, 47514, 47513, 35834, 40340] {
        let services = services(structure.into());
        sqlx::query!("
                INSERT INTO structure_service
                (
                    structure_type_id,
                    service_type_ids,
                    service_slots
                )
                VALUES ($1, $2, $3)
            ",
                structure,
                &services.into_iter().map(|x| *x).collect::<Vec<_>>(),
                service_slots(structure.into(), &type_dogma),
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
    tracing::debug!("Transaction committed");

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
            excluded: excludes(type_id),
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

fn excludes(
    type_id: TypeId,
) -> Vec<TypeId> {
    match *type_id {
        37146 => vec![37147.into()],
        37147 => vec![37146.into()],
        37148 => vec![37149.into()],
        37149 => vec![37148.into()],
        37150 => vec![37151.into()],
        37151 => vec![37150.into()],
        37152 => vec![43732.into()],
        43732 => vec![37152.into()],
        37153 => vec![43919.into()],
        43919 => vec![37153.into()],
        37154 => vec![37155.into()],
        37155 => vec![37154.into()],
        37156 => vec![37157.into()],
        37157 => vec![37156.into()],
        37158 => vec![37159.into()],
        37159 => vec![37158.into()],
        37160 => vec![37161.into()],
        37161 => vec![37160.into()],
        37162 => vec![37163.into()],
        37163 => vec![37162.into()],
        37164 => vec![37165.into()],
        37165 => vec![37164.into()],
        37166 => vec![37167.into()],
        37167 => vec![37166.into()],
        37168 => vec![37169.into()],
        37169 => vec![37168.into()],
        37170 => vec![37171.into()],
        37171 => vec![37170.into()],
        37172 => vec![37173.into()],
        37173 => vec![37172.into()],
        37174 => vec![37175.into()],
        37175 => vec![37174.into()],
        37178 => vec![37179.into()],
        37179 => vec![37178.into()],
        37180 => vec![37181.into()],
        37181 => vec![37180.into()],
        37182 => vec![37183.into()],
        37183 => vec![37182.into()],
        43704 => vec![43705.into()],
        43705 => vec![43704.into()],
        43707 => vec![43708.into()],
        43708 => vec![43707.into()],
        43709 => vec![43711.into()],
        43711 => vec![43709.into()],
        43712 => vec![43713.into()],
        43713 => vec![43712.into()],
        43714 => vec![43715.into()],
        43715 => vec![43714.into()],
        43716 => vec![43717.into()],
        43717 => vec![43716.into()],
        43718 => vec![43719.into()],
        43719 => vec![43718.into()],
        43720 => vec![43721.into()],
        43721 => vec![43720.into()],
        43722 => vec![43723.into()],
        43723 => vec![43722.into()],
        43724 => vec![43725.into()],
        43725 => vec![43724.into()],
        43726 => vec![43727.into()],
        43727 => vec![43726.into()],
        43729 => vec![43730.into()],
        43730 => vec![43729.into()],
        43733 => vec![43734.into()],
        43734 => vec![43733.into()],
        43854 => vec![43855.into()],
        43855 => vec![43854.into()],
        43856 => vec![43857.into()],
        43857 => vec![43856.into()],
        43858 => vec![43859.into()],
        43859 => vec![43858.into()],
        43860 => vec![43861.into()],
        43861 => vec![43860.into()],
        43862 => vec![43863.into()],
        43863 => vec![43862.into()],
        43864 => vec![43865.into()],
        43865 => vec![43864.into()],
        43866 => vec![43867.into()],
        43867 => vec![43866.into()],
        43868 => vec![43869.into()],
        43869 => vec![43868.into()],
        43870 => vec![43871.into()],
        43871 => vec![43870.into()],
        43872 => vec![43873.into()],
        43873 => vec![43872.into()],
        43874 => vec![43875.into()],
        43875 => vec![43874.into()],
        43876 => vec![43877.into()],
        43877 => vec![43876.into()],
        43878 => vec![43879.into()],
        43879 => vec![43878.into()],
        43880 => vec![43881.into()],
        43881 => vec![43880.into()],
        43882 => vec![43883.into()],
        43883 => vec![43882.into()],
        43884 => vec![43885.into()],
        43885 => vec![43884.into()],
        43886 => vec![43887.into()],
        43887 => vec![43886.into()],
        43888 => vec![43889.into()],
        43889 => vec![43888.into()],
        43890 => vec![43891.into()],
        43891 => vec![43890.into()],
        43892 => vec![43893.into()],
        43893 => vec![43892.into()],
        43920 => vec![43921.into()],
        43921 => vec![43920.into()],
        45544 => vec![],
        45546 => vec![],
        45548 => vec![],
        45640 => vec![],
        45641 => vec![],
        46323 => vec![46324.into()],
        46324 => vec![46323.into()],
        46325 => vec![46326.into()],
        46326 => vec![46325.into()],
        46327 => vec![46328.into()],
        46328 => vec![46327.into()],
        46484 => vec![46485.into()],
        46485 => vec![46484.into()],
        46486 => vec![46487.into()],
        46487 => vec![46486.into()],
        46488 => vec![46489.into()],
        46489 => vec![46488.into()],
        46490 => vec![46491.into()],
        46491 => vec![46490.into()],
        46492 => vec![46493.into()],
        46493 => vec![46492.into()],
        46494 => vec![46495.into()],
        46495 => vec![46494.into()],
        46496 => vec![46497.into()],
        46497 => vec![46496.into()],
        46633 => vec![46634.into()],
        46634 => vec![46633.into()],
        46635 => vec![46636.into()],
        46636 => vec![46635.into()],
        46637 => vec![46638.into()],
        46638 => vec![46637.into()],
        46639 => vec![46639.into()],
        46640 => vec![46639.into()],
        46641 => vec![46642.into()],
        46642 => vec![46641.into()],
        _     => vec![],
    }
}

fn services(
    structure_type_id: TypeId,
) -> Vec<TypeId> {
    match *structure_type_id {
            35825 => vec![35894, 35891, 45550, 35886, 35878, 35899],
            35826 => vec![35894, 35892, 35891, 45550, 35886, 35878, 35881, 35899],
            35827 => vec![35894, 35892, 35891, 45550, 35886, 35878, 35881, 35877, 35899],
            35835 => vec![35894, 35891, 45550, 35886, 35878, 45009, 35899, 45539, 45537, 45538],
            35836 => vec![35894, 35892, 35891, 45550, 35886, 35878, 45009, 35899, 45539, 45537, 45538],
            35832 => vec![35894, 35891, 45550, 35886, 35878, 35899],
            // Fortizar
            35833 |
            // 'Prometheus' Fortizar
            47516 |
            // 'Moreau' Fortizar
            47512 |
            // 'Marginis' Fortizar
            47515 |
            // 'Horizon' Fortizar
            47514 |
            // 'Draccous' Fortizar
            47513  => vec![35894, 35892, 35891, 45550, 35886, 35878, 35881, 35899],
            // Keepstar
            35834 |
            // Upwell Palatine Keepstar
            40340 => vec![35894, 35892, 35891, 45550, 35886, 35878, 35881, 35899],
            _     => Vec::new(),
        }
        .into_iter()
        .map(Into::into)
        .collect::<Vec<_>>()
}

fn service_slots(
    structure_type_id: TypeId,
    type_dogma:        &HashMap<usize, TypeDogma>,
) -> i32 {
    let dogma = if let Some(x) = type_dogma.get(&(*structure_type_id as usize)) {
        x
    } else {
        return 0i32;
    };

    dogma
        .attributes
        .iter()
        .find(|x| x.attribute_id == 2056)
        .map(|x| x.value as i32)
        .unwrap_or_default()
}

#[derive(Debug)]
struct Structure {
    type_id:            TypeId,
    structure_type_ids: Vec<TypeId>,
    excluded:           Vec<TypeId>,
}
