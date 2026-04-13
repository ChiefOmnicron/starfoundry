use serde::Deserialize;
use sqlx::PgPool;
use starfoundry_lib_market::{Asteroid, Gas, GasDecompressionEfficiency, MarketApiClient, MarketVirtualRequest, OreReprocessingEfficiency};
use starfoundry_lib_types::{StructureId, TypeId};
use std::collections::HashMap;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::project::error::{ProjectError, Result};
use crate::project::ProjectUuid;

pub async fn update_market_bulk(
    pool:               &PgPool,
    project_id:         ProjectUuid,
    entries:            Vec<UpdateProjectMarket>,
    market_api_client:  &impl MarketApiClient,
) -> Result<()> {
    let market_entries = sqlx::query!("
            SELECT
                id,
                quantity,
                type_id
            FROM project_market
            WHERE project_id = $1
            AND cost IS NULL
            AND quantity > 0
        ",
            *project_id,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| ProjectError::Fetch(e, project_id))?
        .into_iter()
        .map(|x| (x.type_id, TmpMarketEntry {
            id:         x.id,
            quantity:   x.quantity,
        }))
        .collect::<HashMap<_, _>>();

    // adds a new entry
    let mut new_entries = Vec::new();
    // updates the full entry
    let mut update_entries = Vec::new();
    // only updates the quantity and nothing else
    let mut update_quantity = Vec::new();
    // deletes the order_id
    let mut delete_entries = Vec::new();
    // adds the entries as excess
    let mut excess_entries = Vec::new();

    let mut mineral_updates: HashMap<TypeId, i32> = HashMap::new();

    for entry in entries.iter() {
        if let Ok(asteroid) = Asteroid::try_from(entry.type_id) {
            let compression_efficiency = entry
                .mineral_compression
                .unwrap_or(OreReprocessingEfficiency::default())
                .efficiency();

            asteroid
                .minerals()
                .into_iter()
                .map(|(mineral, x)| (mineral.to_type_id(), ((x * (entry.quantity as f64 / 100f64)) * compression_efficiency).floor() as i32))
                .collect::<HashMap<_, _>>()
                .into_iter()
                .for_each(|(type_id, x)| {
                    mineral_updates
                        .entry(type_id)
                        .and_modify(|y: &mut i32| *y += x)
                        .or_insert(x);
                });

            new_entries.push(entry);
        }

        let market_entry = if let Some(x) = market_entries.get(&entry.type_id) {
            x
        } else {
            if Gas::is_gas(entry.type_id) {
                let gas = Gas::from(entry.type_id);
                if gas.is_compressed() {
                    let uncompressed = Gas::from(entry.type_id).to_uncompressed_type_id();

                    let decompression_efficiency = entry
                        .gas_decompression
                        .unwrap_or(GasDecompressionEfficiency::default())
                        .efficiency();
                    let original_amount = (
                        entry.quantity as f64 * decompression_efficiency
                    ).floor();
                    // adds the compressed amount
                    new_entries.push(entry);

                    if let Some(x) = market_entries.get(&uncompressed) {
                        if original_amount as i32 >= x.quantity {
                            delete_entries.push(x.id);
                        } else {
                            update_quantity.push(TmpMarketEntry {
                                id:         x.id,
                                quantity:   x.quantity - original_amount as i32,
                            });
                        }
                    }
                }
                continue;
            }

            continue;
        };

        if entry.quantity < market_entry.quantity {
            update_quantity.push(TmpMarketEntry {
                id:         market_entry.id,
                quantity:   market_entry.quantity - entry.quantity,
            });
            new_entries.push(entry);
        } else if entry.quantity >= market_entry.quantity {
            update_entries.push(TmpUpdateEntry {
                id:         market_entry.id,
                cost:       entry.cost,
                quantity:   entry.quantity,
                source:     entry.source.clone(),
            });
        }
    }

    for (type_id, quantity) in mineral_updates {
        let market_entry = if let Some(x) = market_entries.get(&type_id) {
            x
        } else {
            continue;
        };

        if quantity < market_entry.quantity {
            update_quantity.push(TmpMarketEntry {
                id:         market_entry.id,
                quantity:   market_entry.quantity - quantity,
            });
        } else if quantity >= market_entry.quantity {
            excess_entries.push(TmpExcessEntry {
                type_id:    type_id,
                quantity:   quantity - market_entry.quantity,
            });
            delete_entries.push(market_entry.id);
        }
    }

    let mut transaction = pool
        .begin()
        .await
        .map_err(ProjectError::TransactionError)?;

    if !new_entries.is_empty() {
        let type_ids = new_entries
            .iter()
            .map(|x| *x.type_id)
            .collect::<Vec<_>>();
        let quantities = new_entries
            .iter()
            .map(|x| x.quantity)
            .collect::<Vec<_>>();
        let costs = new_entries
            .iter()
            .map(|x| x.cost as f64)
            .collect::<Vec<_>>();
        let sources = new_entries
            .iter()
            .map(|x| x.source.clone())
            .collect::<Vec<_>>();

        sqlx::query!("
                INSERT INTO project_market (
                    project_id,
                    type_id,
                    quantity,
                    cost,
                    source
                )
                SELECT $1, * FROM UNNEST(
                    $2::INTEGER[],
                    $3::INTEGER[],
                    $4::DOUBLE PRECISION[],
                    $5::VARCHAR[]
                )
            ",
                *project_id,
                &type_ids,
                &quantities,
                &costs,
                &sources,
            )
            .execute(&mut *transaction)
            .await
            .map_err(ProjectError::Update)?;
    }

    if !update_quantity.is_empty() {
            sqlx::query!("
                UPDATE project_market AS pm
                SET quantity = update.quantity
                FROM UNNEST(
                    $1::UUID[],
                    $2::INTEGER[]
                ) as update(id, quantity)
                WHERE pm.id = update.id
            ",
                &update_quantity.iter().map(|x| x.id).collect::<Vec<_>>(),
                &update_quantity.iter().map(|x| x.quantity).collect::<Vec<_>>(),
            )
            .execute(&mut *transaction)
            .await
            .map_err(ProjectError::Update)?;
    }

    if !update_entries.is_empty() {
        sqlx::query!("
                UPDATE project_market AS pm
                SET
                    source = update.source,
                    quantity = update.quantity,
                    cost = update.cost
                FROM UNNEST(
                    $1::UUID[],
                    $2::VARCHAR[],
                    $3::INTEGER[],
                    $4::DOUBLE PRECISION[]
                ) as update(id, source, quantity, cost)
                WHERE pm.id = update.id
            ",
                &update_entries.iter().map(|x| x.id).collect::<Vec<_>>(),
                &update_entries.iter().map(|x| x.source.clone()).collect::<Vec<_>>(),
                &update_entries.iter().map(|x| x.quantity).collect::<Vec<_>>(),
                &update_entries.iter().map(|x| x.cost as f64).collect::<Vec<_>>(),
            )
            .execute(&mut *transaction)
            .await
            .map_err(ProjectError::Update)?;
    }

    if !excess_entries.is_empty() {
        sqlx::query!("
                INSERT INTO project_excess (
                    type_id,
                    quantity
                )
                SELECT * FROM UNNEST(
                    $1::INTEGER[],
                    $2::INTEGER[]
                )
            ",
                &excess_entries.iter().map(|x| *x.type_id).collect::<Vec<_>>(),
                &excess_entries.iter().map(|x| x.quantity).collect::<Vec<_>>(),
            )
            .execute(&mut *transaction)
            .await
            .map_err(ProjectError::Update)?;
    }

    transaction
        .commit()
        .await
        .map_err(ProjectError::TransactionError)?;

    let virtual_updates = entries
        .iter()
        .filter(|x| x.structure_id.is_some())
        .map(|x| MarketVirtualRequest {
            // unwrap is safe, as it's checked before
            market: x.structure_id.unwrap(),
            quantity: x.quantity,
            type_id: x.type_id,
        })
        .collect::<Vec<_>>();
    if let Err(e) = market_api_client
        .update_virtual_market(virtual_updates)
        .await {

        tracing::error!("{}", e.to_string());
    };

    Ok(())
}

#[derive(Clone, Debug, Deserialize, ToSchema)]
pub struct UpdateProjectMarket {
    pub type_id:                TypeId,
    pub cost:                   f32,
    pub source:                 String,
    pub quantity:               i32,

    // if the structure id is known
    pub structure_id:           Option<StructureId>,
    // gas decompression is active
    pub gas_decompression:      Option<GasDecompressionEfficiency>,
    // mineral compression is active
    pub mineral_compression:    Option<OreReprocessingEfficiency>,
}

#[derive(Debug)]
struct TmpMarketEntry {
    id:         Uuid,
    quantity:   i32,
}

#[derive(Debug)]
struct TmpExcessEntry {
    type_id:    TypeId,
    quantity:   i32,
}

#[derive(Debug)]
struct TmpUpdateEntry {
    id:         Uuid,
    cost:       f32,
    source:     String,
    quantity:   i32,
}
