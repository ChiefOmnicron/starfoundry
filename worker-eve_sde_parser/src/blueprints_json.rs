use serde::Serialize;
use sqlx::PgPool;
use starfoundry_lib_types::{CategoryId, GroupId, TypeId};
use std::collections::{HashMap, VecDeque};
use std::time::Instant;
use starfoundry_lib_eve_gateway::Item;

use crate::Error;
use crate::parser::blueprints::BlueprintEntry;
use crate::parser::groups::GroupIdEntry;
use crate::parser::type_ids::TypeIdEntry;
use crate::items::get_item;
use crate::parser::categories::CategoryIdEntry;

pub async fn run(
    pool:         &PgPool,
    blueprints:   &HashMap<TypeId, BlueprintEntry>,
    category_ids: &HashMap<CategoryId, CategoryIdEntry>,
    group_ids:    &HashMap<GroupId, GroupIdEntry>,
    type_ids:     &HashMap<TypeId, TypeIdEntry>,
    repackaged:   &HashMap<TypeId, i32>,
) -> Result<(), Error> {
    tracing::info!("Processing blueprints");
    let start = Instant::now();

    insert_into_database(
            &pool,
            &blueprints,
            &category_ids,
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
    pool:         &PgPool,
    blueprints:   &HashMap<TypeId, BlueprintEntry>,
    category_ids: &HashMap<CategoryId, CategoryIdEntry>,
    group_ids:    &HashMap<GroupId, GroupIdEntry>,
    type_ids:     &HashMap<TypeId, TypeIdEntry>,
    repackaged:   &HashMap<TypeId, i32>,
) -> Result<(), Error> {
    let products = crate::parser::blueprints::product_type_id_as_key(
        &blueprints,
        &type_ids,
    );

    let find_blueprint_type_id = |product_type_id: TypeId| {
        blueprints
            .iter()
            .filter(|(_, x)| x.product().is_some())
            .find(|(_, x)| x.product().unwrap() == product_type_id)
            .map(|(y, _)| y)
            .unwrap()
            .clone()
    };

    let mut entries: HashMap<TypeId, Dependency> = HashMap::new();
    let mut queue: VecDeque<Dependency> = VecDeque::new();

    for (product_type_id, pentry) in products.iter() {
        if let None = type_ids.get(&product_type_id) {
            continue;
        }

        let ientry = type_ids.get(&product_type_id).unwrap();
        if !ientry.published {
            continue;
        }

        let typ = if pentry.is_reaction() {
            BlueprintTyp::Reaction
        } else {
            BlueprintTyp::Blueprint
        };

        let mut dependency = Dependency {
            product_type_id: *product_type_id,
            blueprint_type_id: find_blueprint_type_id(*product_type_id),
            item: get_item(
                *product_type_id,
                category_ids,
                group_ids,
                type_ids,
                repackaged,
            ),
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
                let dependency = Dependency {
                    product_type_id: material.type_id,
                    blueprint_type_id: 0.into(),
                    time: 0f32,
                    needed: material.quantity as f32,
                    produces: 0,
                    item: get_item(
                        material.type_id,
                        category_ids,
                        group_ids,
                        type_ids,
                        repackaged,
                    ),
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
            entries.insert(*product_type_id, dependency);
        } else {
            queue.push_back(dependency);
        }
    }

    while let Some(pentry) = queue.pop_front() {
        let mut entry = pentry;
        let materials = products.get(&entry.product_type_id).unwrap().materials();

        let mut components = Vec::new();
        for material in materials.iter() {
            if products.contains_key(&material.type_id) && entries.contains_key(&material.type_id) {
                let mut entry = entries.get(&material.type_id).unwrap().clone();
                entry.needed = material.quantity as f32;
                components.push(entry);
            } else if !products.contains_key(&material.type_id) {
                let dependency = Dependency {
                    product_type_id: material.type_id.into(),
                    blueprint_type_id: 0.into(),
                    time: 0f32,
                    needed: material.quantity as f32,
                    produces: 0,
                    item: get_item(
                        material.type_id,
                        category_ids,
                        group_ids,
                        type_ids,
                        repackaged,
                    ),
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
            entries.insert(entry.product_type_id, entry);
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

    let mut blueprint_type_ids = Vec::new();
    let mut product_type_ids = Vec::new();
    let mut json      = Vec::new();

    for (_, entry) in entries {
        blueprint_type_ids.push(*entry.blueprint_type_id);
        product_type_ids.push(*entry.product_type_id);
        json.push(serde_json::to_value(&entry).unwrap());
    }

    tracing::debug!("Inserting data");
    sqlx::query!("
            INSERT INTO blueprint_json
            (
                blueprint_type_id,
                product_type_id,
                data
            )
            SELECT * FROM UNNEST(
                $1::INTEGER[],
                $2::INTEGER[],
                $3::JSON[]
            )
        ",
            &blueprint_type_ids,
            &product_type_ids,
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
    blueprint_type_id: TypeId,
    blueprint_name: String,
    product_type_id: TypeId,
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

#[derive
(
    Copy, Clone, Debug,
    Eq, PartialEq,
    Serialize,
)]
pub enum BlueprintTyp {
    Blueprint,
    Reaction,
    Material,
}

#[derive(Clone, Debug, Serialize)]
pub struct Dependency {
    pub blueprint_type_id: TypeId,
    pub product_type_id:   TypeId,
    pub needed:            f32,
    pub time:              f32,
    pub produces:          i32,
    pub item:              Item,
    pub components:        Vec<Dependency>,
    pub typ:               BlueprintTyp,
}
