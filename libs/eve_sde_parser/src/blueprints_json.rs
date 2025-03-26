use sqlx::PgPool;
use starfoundry_libs_items::Item;
use starfoundry_libs_projects::{BlueprintTyp, Dependency};
use starfoundry_libs_types::{TypeId, GroupId};
use std::collections::{HashMap, VecDeque};
use std::time::Instant;

use crate::Error;
use crate::parser::blueprints::BlueprintEntry;
use crate::parser::group_ids::GroupIdEntry;
use crate::parser::type_ids::TypeIdEntry;

pub async fn run(
    pool:       &PgPool,
    blueprints: &HashMap<TypeId, BlueprintEntry>,
    group_ids:  &HashMap<GroupId, GroupIdEntry>,
    type_ids:   &HashMap<TypeId, TypeIdEntry>,
    repackaged: &HashMap<TypeId, i32>,
) -> Result<(), Error> {
    tracing::info!("Processing blueprints");
    let start = Instant::now();

    insert_into_database(
            &pool,
            &blueprints,
            &group_ids,
            &type_ids,
            &repackaged,
        )
        .await?;

    tracing::info!(
        "Finished processing blueprints, task took {:.2}s",
        start.elapsed().as_secs_f64()
    );

    Ok(())
}

async fn insert_into_database(
    pool:       &PgPool,
    blueprints: &HashMap<TypeId, BlueprintEntry>,
    groups_ids: &HashMap<GroupId, GroupIdEntry>,
    type_ids:   &HashMap<TypeId, TypeIdEntry>,
    repackaged: &HashMap<TypeId, i32>,
) -> Result<(), Error> {
    let products = crate::parser::blueprints::product_type_id_as_key(
        &blueprints,
        &type_ids,
    );

    let find_btype_id = |ptype_id: TypeId| {
        blueprints
            .iter()
            .filter(|(_, x)| x.product().is_some())
            .find(|(_, x)| x.product().unwrap() == ptype_id)
            .map(|(y, _)| y)
            .unwrap()
            .clone()
    };

    let mut entries: HashMap<TypeId, Dependency> = HashMap::new();
    let mut queue: VecDeque<Dependency> = VecDeque::new();

    for (ptype_id, pentry) in products.iter() {
        if let None = type_ids.get(&ptype_id) {
            continue;
        }

        let ientry = type_ids.get(&ptype_id).unwrap();
        if !ientry.published {
            continue;
        }

        let iname = ientry
            .name()
            .unwrap_or_default()
            .replace('\'', "''");
        let igroup_id = ientry.group_id;
        let icategory_id = groups_ids.get(&igroup_id).unwrap().category_id;
        let imeta_group_id = ientry.meta_group_id;
        let irepackaged = repackaged.get(&ptype_id).cloned();

        let typ = if pentry.is_reaction() {
            BlueprintTyp::Reaction
        } else {
            BlueprintTyp::Blueprint
        };

        let mut dependency = Dependency {
            ptype_id: *ptype_id,
            btype_id: find_btype_id(*ptype_id),
            item: Item {
                name: iname.clone(),
                volume: ientry.volume.unwrap_or_default(),
                category_id: icategory_id.into(),
                group_id: igroup_id.into(),
                type_id: *ptype_id,
                meta_group_id: imeta_group_id.map(Into::into),
                repackaged: irepackaged,
            },
            needed: 0f32,
            time: pentry.manufacture_time().unwrap() as f32,
            produces: pentry.product_quantity().unwrap(),
            typ: typ,
            components: Vec::new(),
        };

        let mut components = Vec::new();
        for material in pentry.materials() {
            if products.contains_key(&material.type_id) && entries.contains_key(&material.type_id) {
                let mut entry = entries.get(&material.type_id).unwrap().clone();
                entry.needed = material.quantity as f32;
                components.push(entry);
            } else if !products.contains_key(&material.type_id) {
                let ientry = type_ids.get(&material.type_id).unwrap();
                let iname = ientry
                    .name()
                    .unwrap_or_default()
                    .replace('\'', "''");
                let igroup_id = ientry.group_id;
                let icategory_id = groups_ids.get(&igroup_id).unwrap().category_id;
                let imeta_group_id = ientry.meta_group_id;
                let irepackaged = repackaged.get(&ptype_id).cloned();

                let dependency = Dependency {
                    ptype_id: material.type_id,
                    btype_id: 0.into(),
                    time: 0f32,
                    needed: material.quantity as f32,
                    produces: 0,
                    item: Item {
                        name: iname.clone(),
                        volume: ientry.volume.unwrap_or_default(),
                        category_id: icategory_id.into(),
                        group_id: igroup_id.into(),
                        type_id: *ptype_id,
                        meta_group_id: imeta_group_id.map(Into::into),
                        repackaged: irepackaged,
                    },
                    typ: BlueprintTyp::Material,
                    components: Vec::new(),
                };
                components.push(dependency);
            } else {
                queue.push_back(dependency.clone());
                break;
            }
        }

        if components.len() == pentry.materials().len() {
            dependency.components = components;
            entries.insert(*ptype_id, dependency);
        } else {
            queue.push_back(dependency);
        }
    }

    while let Some(pentry) = queue.pop_front() {
        let mut entry = pentry;
        let materials = products.get(&entry.ptype_id).unwrap().materials();

        let mut components = Vec::new();
        for material in materials.iter() {
            if products.contains_key(&material.type_id) && entries.contains_key(&material.type_id) {
                let mut entry = entries.get(&material.type_id).unwrap().clone();
                entry.needed = material.quantity as f32;
                components.push(entry);
            } else if !products.contains_key(&material.type_id) {
                let ientry = type_ids.get(&material.type_id).unwrap();
                let iname = ientry
                    .name()
                    .unwrap_or_default()
                    .replace('\'', "''");
                let igroup_id = ientry.group_id;
                let icategory_id = groups_ids.get(&igroup_id).unwrap().category_id;
                let imeta_group_id = ientry.meta_group_id;
                let irepackaged = repackaged.get(&material.type_id).cloned();

                let dependency = Dependency {
                    ptype_id: material.type_id.into(),
                    btype_id: 0.into(),
                    time: 0f32,
                    needed: material.quantity as f32,
                    produces: 0,
                    item: Item {
                        name: iname.clone(),
                        volume: ientry.volume.unwrap_or_default(),
                        category_id: icategory_id.into(),
                        group_id: igroup_id.into(),
                        type_id: material.type_id.into(),
                        meta_group_id: imeta_group_id.map(Into::into),
                        repackaged: irepackaged,
                    },
                    typ: BlueprintTyp::Material,
                    components: Vec::new(),
                };
                components.push(dependency);
            } else {
                queue.push_back(entry.clone());
                break;
            }
        }

        if components.len() == materials.len() {
            entry.components = components;
            entries.insert(entry.ptype_id, entry);
        } else {
            queue.push_back(entry);
        }
    }

    let mut transaction = pool
        .begin()
        .await
        .map_err(Error::TransactionError)?;

    tracing::debug!("Clearing blueprint_json database");
    sqlx::query!("
            DELETE FROM blueprint_json
        ")
        .execute(&mut *transaction)
        .await
        .map_err(Error::DeleteBlueprintJson)?;
    tracing::debug!("Clearing blueprint_json database done");

    let mut btype_ids = Vec::new();
    let mut ptype_ids = Vec::new();
    let mut json      = Vec::new();

    for (_, entry) in entries {
        btype_ids.push(*entry.btype_id);
        ptype_ids.push(*entry.ptype_id);
        json.push(serde_json::to_value(&entry).unwrap());
    }

    tracing::debug!("Inserting data");
    sqlx::query!("
            INSERT INTO blueprint_json
            (
                btype_id,
                ptype_id,
                data
            )
            SELECT * FROM UNNEST(
                $1::INTEGER[],
                $2::INTEGER[],
                $3::JSON[]
            )
        ",
            &btype_ids,
            &ptype_ids,
            &json
        )
        .execute(&mut *transaction)
        .await
        .map_err(Error::InsertBlueprintJson)?;
    tracing::debug!("Inserting data done");

    transaction
        .commit()
        .await
        .map_err(Error::TransactionError)?;
    tracing::debug!("Transaction commited");
    Ok(())
}

/*#[derive(Clone, Debug, Serialize)]
struct Dependency {
    btype_id: TypeId,
    blueprint_name: String,
    ptype_id: TypeId,
    time: u32,
    quantity: u32,
    produces: u32,
    item: Item,
    typ: DependencyType,
    components: Vec<Dependency>,
}

#[derive(Clone, Debug, Serialize)]
struct DependencyInfo {
    type_id: TypeId,
    category_id: u32,
    group_id: u32,
    meta_group_id: Option<i32>,
    name: String,
}
*/
