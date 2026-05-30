use serde::Deserialize;
use sqlx::PgPool;
use starfoundry_lib_industry::ProjectUuid;
use starfoundry_lib_market::{Asteroid, Gas, GasDecompressionEfficiency, MarketApiClient, MarketVirtualRequest, OreReprocessingEfficiency};
use starfoundry_lib_types::{StructureId, TypeId};
use std::collections::HashMap;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::project::error::{ProjectError, Result};

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
    let mut moon_updates: HashMap<TypeId, i32> = HashMap::new();

    for entry in entries.iter() {
        if let Ok(asteroid) = Asteroid::try_from_asteroid(entry.type_id) {
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
                    // ignore R4 in the calculations, otherwise it will cause
                    // issues down stream.
                    // -> Startable jobs doesn't detect them, as they are not
                    //    in the stock or market
                    // -> It is possible to buy the R4 needed + it is getting
                    //    removed due to it being in the Asteroids bought
                    // TODO: implement it into the asteroid calculations
                    //       and properly record the change in the market part
                    if
                        type_id == Asteroid::AtmosphericGases.to_type_id() ||
                        type_id == Asteroid::EvaporiteDeposits.to_type_id() ||
                        type_id == Asteroid::Hydrocarbons.to_type_id() ||
                        type_id == Asteroid::Silicates.to_type_id() {

                        moon_updates
                            .entry(type_id)
                            .and_modify(|y: &mut i32| *y += x)
                            .or_insert(x);
                    } else {
                        mineral_updates
                            .entry(type_id)
                            .and_modify(|y: &mut i32| *y += x)
                            .or_insert(x);
                    }
                });

            new_entries.push(entry);
            continue;
        }

        let market_entry = if let Some(x) = market_entries.get(&entry.type_id) {
            x
        } else {
            if let Ok(gas) = Gas::try_from(entry.type_id) {
                if gas.is_compressed() {
                    let uncompressed_type_id = gas.to_uncompressed_type_id();

                    let decompression_quantity = entry
                        .gas_decompression
                        .unwrap_or(GasDecompressionEfficiency::default())
                        .decompression_quantity(entry.quantity);

                    // adds the compressed amount
                    new_entries.push(entry);

                    if let Some(x) = market_entries.get(&uncompressed_type_id) {
                        if decompression_quantity >= x.quantity {
                            delete_entries.push(x.id);
                        } else {
                            update_quantity.push(TmpMarketEntry {
                                id:         x.id,
                                quantity:   x.quantity - decompression_quantity,
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

    for (type_id, quantity) in moon_updates {
        excess_entries.push(TmpExcessEntry {
            type_id:    type_id,
            quantity:   quantity,
        });
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

    if !delete_entries.is_empty() {
        sqlx::query!("
                DELETE FROM project_market
                WHERE id = ANY($1)
            ",
                &delete_entries,
            )
            .execute(&mut *transaction)
            .await
            .map_err(ProjectError::Update)?;
    }

    if !excess_entries.is_empty() {
        sqlx::query!("
                INSERT INTO project_excess (
                    project_id,
                    type_id,
                    quantity
                )
                SELECT $1, * FROM UNNEST(
                    $2::INTEGER[],
                    $3::INTEGER[]
                )
                ON CONFLICT (project_id, type_id)
                DO UPDATE SET
                    quantity = project_excess.quantity + EXCLUDED.quantity
            ",
                *project_id,
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
