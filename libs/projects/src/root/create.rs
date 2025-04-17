use sqlx::{PgPool, PgConnection};
use starfoundry_libs_types::{CharacterId, TypeId};
use std::collections::HashMap;
use std::str::FromStr;
use uuid::Uuid;

use crate::{group_structures, AddExcess, AddJobEntry, AddMarket, AddProduct, AdditionalProduct, BlueprintBonus, BlueprintTyp, CreateProject, Error, ProjectUuid, Result, StockMinimal};
use crate::engine::{CalculationEngine, Dependency, DependencyTreeEntry, EngineResult, ProjectConfigBuilder};

pub async fn create(
    pool:         &PgPool,
    character_id: CharacterId,
    project_data: CreateProject,
) -> Result<ProjectUuid> {
    let mut transaction = pool
        .begin()
        .await
        .map_err(Error::TransactionBeginError)?;

    let project_uuid: ProjectUuid = sqlx::query!("
            INSERT INTO projects
            (
                owner,
                name,
                project_group_id,
                structure_group_id,

                sell_price,
                orderer,
                notes
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id
        ",
            *character_id,
            project_data.name,
            *project_data.project_group_id,
            *project_data.structure_group_id,
            project_data.sell_price,
            project_data.orderer,
            project_data.notes,
        )
        .fetch_one(&mut *transaction)
        .await
        .map(|x| x.id.into())
        .map_err(Error::CreateProject)?;

    sqlx::query!("
            INSERT INTO project_market_structures (project_id, structure_id)
            SELECT $1, UNNEST(
                $2::UUID[]
            )
        ",
            *project_uuid,
            &project_data.markets.iter().map(|x| **x).collect::<Vec<_>>(),
        )
        .execute(&mut *transaction)
        .await
        .map_err(Error::CreateProject)?;

    sqlx::query!("
            INSERT INTO project_blacklist (project_id, type_id)
            SELECT $1, UNNEST(
                $2::INTEGER[]
            )
        ",
            *project_uuid,
            &project_data.blacklist.iter().map(|x| **x).collect::<Vec<_>>(),
        )
        .execute(&mut *transaction)
        .await
        .map_err(Error::CreateProject)?;

    let structure_groups = group_structures(
            pool,
            character_id,
            project_data.structure_group_id,
        )
        .await
        .unwrap();

    let blueprint_overwrites = project_data
        .products
        .iter()
        .map(|x| (
            x.type_id,
            BlueprintBonus {
                ptype_id: x.type_id,
                material: x.material_efficiency as f32,
                time:     0f32,
            }
        ))
        .collect::<HashMap<_, _>>();

    let systems = structure_groups
        .iter()
        .map(|x| x.system_ids.iter().map(|x| **x).collect::<Vec<_>>())
        .flatten()
        .collect::<Vec<_>>();
    let system_index = sqlx::query!("
                SELECT
                    system_id,
                    manufacturing,
                    reaction
                FROM industry_index
                WHERE timestamp = (
                    SELECT timestamp
                    FROM industry_index
                    WHERE system_id = ANY($1)
                    GROUP BY system_id, timestamp
                    ORDER BY timestamp DESC
                    LIMIT 1
                )
                AND system_id = ANY($1)
            ",
            &systems
        )
        .fetch_all(pool)
        .await
        .unwrap()
        .into_iter()
        .map(|x| (x.system_id.into(), (x.manufacturing, x.reaction)))
        .collect::<HashMap<_, _>>();

    let material_cost = sqlx::query!("
                SELECT
                    type_id,
                    adjusted_price
                FROM market_prices
            ",
        )
        .fetch_all(pool)
        .await
        .unwrap()
        .into_iter()
        .map(|x| (x.type_id.into(), x.adjusted_price))
        .collect::<HashMap<_, _>>();

    let mut selected_tree: Option<EngineResult> = None;
    let mut selected_tree_total_cost = f32::INFINITY;

    let mut stock_filtered = HashMap::new();
    project_data
        .stocks
        .iter()
        .for_each(|x| {
            stock_filtered
                .entry(x.type_id)
                .and_modify(|y| *y += x.quantity)
                .or_insert(x.quantity);
        });
    let stock_filtered = stock_filtered
        .into_iter()
        .map(|(type_id, quantity)| StockMinimal {
            quantity,
            type_id
        })
        .collect::<Vec<_>>();

    let mut max_runs = HashMap::new();
    if project_data.project_group_id == Uuid::from_str("ec3a9410-ee92-4435-925d-b81a7a987891").unwrap_or_default().into() {
        max_runs.insert(21009.into(), 40);
        max_runs.insert(21011.into(), 40);
        max_runs.insert(21013.into(), 40);
        max_runs.insert(21017.into(), 40);
        max_runs.insert(21019.into(), 40);
        max_runs.insert(21037.into(), 40);
        max_runs.insert(24545.into(), 40);
        max_runs.insert(24547.into(), 40);
        max_runs.insert(24556.into(), 40);
        max_runs.insert(24558.into(), 40);
        max_runs.insert(24560.into(), 40);
        max_runs.insert(24545.into(), 40);
        max_runs.insert(21021.into(), 40);
        max_runs.insert(21023.into(), 40);
        max_runs.insert(21025.into(), 40);
        max_runs.insert(21035.into(), 40);
        max_runs.insert(57487.into(), 40);
        max_runs.insert(57479.into(), 40);
        max_runs.insert(57474.into(), 40);
        max_runs.insert(57486.into(), 200);
        max_runs.insert(57478.into(), 200);
        max_runs.insert(57470.into(), 200);
    }

    for structure_group in structure_groups {
        let config = ProjectConfigBuilder::default()
            .add_blacklists(project_data.blacklist.clone())
            .add_blueprint_overwrites(blueprint_overwrites.clone())
            .add_structures(structure_group.structures)
            .add_structure_mappings(structure_group.mapping)
            .set_material_cost(material_cost.clone())
            .set_system_index(system_index.clone())
            .set_max_runs(max_runs.clone())
            .build();

        let mut dependency_tree = CalculationEngine::new(config);

        for dependency in project_data.products.iter() {
            let type_id = dependency.type_id;
            let json = sqlx::query!("
                    SELECT data
                    FROM   blueprint_json
                    WHERE  ptype_id = $1
                ",
                    *type_id
                )
                .fetch_one(pool)
                .await;

            let json = if let Ok(x) = json {
                x.data
            } else {
                return Err(Error::InvalidTypeId(type_id))
            };
            let json = serde_json::to_value(&json).unwrap();

            if let Ok(x) = Dependency::try_from(dependency.quantity, json) {
                dependency_tree.add(x);
            } else {
                continue;
            };
        }

        let dependency_result = dependency_tree
            .apply_bonus()
            .add_stocks(&stock_filtered)
            .finalize();

        let total_cost = dependency_result.total_cost();

        if total_cost < selected_tree_total_cost {
            selected_tree = Some(dependency_result);
            selected_tree_total_cost = total_cost;
        }
    }

    let (tree, stocks) = if let Some(x) = selected_tree {
        (x.tree, x.stocks)
    } else {
        return Err(Error::NoValidStructureGroup);
    };

    let mut stocks = stocks
        .into_iter()
        .filter(|x| x.quantity > 0)
        .collect::<Vec<_>>();
    stocks.sort_by_key(|x| x.type_id);
    stocks.dedup();

    let mut additional_products_filtered = HashMap::new();
    project_data
        .additional_products
        .iter()
        .for_each(|x| {
            additional_products_filtered
                .entry(x.type_id)
                .and_modify(|y| *y += x.quantity)
                .or_insert(x.quantity);
        });

    let mut additional_products = Vec::new();
    for (type_id, quantity) in additional_products_filtered.iter() {
        if let Some(x) = stock_filtered.iter().find(|x| x.type_id == *type_id) {
            if x.quantity as u32 > *quantity {
                stocks.push(StockMinimal {
                    type_id: *type_id,
                    quantity: *quantity as i32,
                });
            } else if x.quantity as u32 == *quantity {
                stocks.push(StockMinimal {
                    type_id: *type_id,
                    quantity: *quantity as i32,
                });
            } else if (x.quantity as u32) < *quantity {
                stocks.push(StockMinimal {
                    type_id: *type_id,
                    quantity: x.quantity,
                });
                additional_products
                    .push(AdditionalProduct {
                        quantity: quantity - x.quantity as u32,
                        type_id: *type_id,
                    });
            }
        } else {
            additional_products
                .push(AdditionalProduct {
                    quantity: *quantity,
                    type_id: *type_id,
                });
        }
    }

    insert_jobs(&mut *transaction, &project_uuid, &tree).await?;
    insert_market(&mut *transaction, &project_uuid, &tree, &additional_products).await?;
    insert_excess(&mut *transaction, &project_uuid, &tree).await?;
    insert_products(&mut *transaction, &project_uuid, project_data.products).await?;
    insert_stock(&mut *transaction, &project_uuid, stocks).await?;

    transaction
        .commit()
        .await
        .map_err(Error::TransactionCommitError)?;

    Ok(project_uuid)
}

async fn insert_jobs(
    transaction:         &mut PgConnection,
    project_uuid:        &ProjectUuid,
    tree:                &HashMap<TypeId, DependencyTreeEntry>,
) -> Result<()> {
    let mut entries = Vec::new();

    for (_, entry) in tree
        .iter()
        .filter(|(_, x)| x.typ != BlueprintTyp::Material
    ) {
        let jobs = entry
            .runs
            .iter()
            .map(|x| {
                let structure_id = if let Some(x) = &entry.structure {
                    x.id
                } else {
                    Uuid::default().into()
                };

                AddJobEntry {
                    runs: *x as i32,
                    structure_id: structure_id,
                    type_id: entry.product_type_id,
                }
            })
            .collect::<Vec<_>>();
        entries.extend(jobs);
    }

    crate::job::add_with_transaction(&mut *transaction, project_uuid, entries).await
}

async fn insert_market(
    transaction:         &mut PgConnection,
    project_uuid:        &ProjectUuid,
    tree:                &HashMap<TypeId, DependencyTreeEntry>,
    additional_products: &Vec<AdditionalProduct>,
) -> Result<()> {
    let mut entries = Vec::new();

    additional_products
        .iter()
        .for_each(|x| {
            let entry = AddMarket {
                cost:     None,
                source:   None,
                quantity: x.quantity as i32,
                type_id:  x.type_id,
            };
            entries.push(entry);
        });

    tree
        .iter()
        .filter(|(_, x)| x.typ == BlueprintTyp::Material)
        .for_each(|(_, x)| {
            let entry = AddMarket {
                cost:     None,
                source:   None,
                quantity: x.needed.ceil() as i32,
                type_id:  x.product_type_id,
            };
            entries.push(entry);
        });

    crate::market::add_with_transaction(transaction, project_uuid, entries).await
}

async fn insert_excess(
    transaction:  &mut PgConnection,
    project_uuid: &ProjectUuid,
    tree:         &HashMap<TypeId, DependencyTreeEntry>,
) -> Result<()> {
    let mut entries = Vec::new();

    tree
        .iter()
        .filter(|(_, x)| x.typ != BlueprintTyp::Material)
        .for_each(|(_, entry)| {
            let total_produced: u32 = entry
                .runs
                .iter()
                .map(|x| x * entry.produces as u32)
                .sum();

            let excess_quantity = total_produced.saturating_sub(entry.needed.ceil() as u32);

            if excess_quantity != 0 {
                entries.push(AddExcess {
                    quantity: excess_quantity as i32,
                    type_id:  entry.product_type_id,
                });
            }
        });

    crate::excess::add_with_transaction(transaction, project_uuid, entries).await
}

async fn insert_stock(
    transaction:  &mut PgConnection,
    project_uuid: &ProjectUuid,
    entries:      Vec<StockMinimal>,
) -> Result<()> {
    crate::stock::add_with_transaction(transaction, project_uuid, entries).await
}

async fn insert_products(
    transaction:  &mut PgConnection,
    project_uuid: &ProjectUuid,
    entries:      Vec<AddProduct>,
) -> Result<()> {
    crate::product::add_with_transaction(transaction, project_uuid, entries).await
}
