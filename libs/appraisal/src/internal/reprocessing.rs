use serde::Deserialize;
use sqlx::PgPool;
use starfoundry_libs_compression::{Asteroid, Gas, GasReprocessingEfficiency, OreReprocessingEfficiency, ScrapReprocessingEfficiency};
use utoipa::ToSchema;

use crate::Result;
use super::Appraisal;
use std::collections::HashMap;
use crate::internal::{create_type_ids, AppraisalOptions};

pub async fn reprocessing(
    pool:    &PgPool,
    code:    String,
    options: ReprocessingOptions,
) -> Result<Option<Appraisal>> {
    // fetch the appraisal
    let appraisal = if let Some(x) = super::fetch(pool, code).await? {
        x
    } else {
        return Ok(None);
    };

    let mut total_reprocessed = HashMap::new();

    for item in appraisal.items {
        // asteroids
        if item.meta.category_id == 25.into() {
            Asteroid::from_type_id(*item.type_id)
                .minerals()
                .into_iter()
                .for_each(|(mineral, quantity)| {
                    // make sure that there is always at least 100
                    let quantity = (((item.quantity as f64) / 100f64).floor()) * quantity;

                    let reprocessed = (
                        (
                            quantity
                        ) * options.ore_reprocessing.efficiency()
                    ).floor() as i64;

                    if reprocessed > 0 {
                        total_reprocessed
                            .entry(mineral.to_type_id())
                            .and_modify(|x: &mut i64| *x += reprocessed)
                            .or_insert(reprocessed);
                    }
                });
        // compressed gas
        } else if item.meta.group_id == 4168.into() {
            let reprocessed = (
                (
                    item.quantity as f64
                ) * (options.gas_decompression.efficiency() / 100f64)
            ).floor() as i64;

            if reprocessed > 0 {
                total_reprocessed
                    .entry(Gas::from_type_id(*item.type_id).to_uncompressed_type_id().into())
                    .and_modify(|x: &mut i64| *x += reprocessed)
                    .or_insert(reprocessed);
            }
        } else if
            // minerals
            item.meta.group_id == 18.into() ||
            // moon materials
            item.meta.group_id == 427.into() {

            total_reprocessed
                .entry(item.type_id.into())
                .and_modify(|value: &mut i64| *value += item.quantity)
                .or_insert(item.quantity);
        } else if item.meta.group_id == 711.into() {
            // gas
            total_reprocessed
                .entry(item.type_id.into())
                .and_modify(|value: &mut i64| *value += item.quantity)
                .or_insert(item.quantity);
        } else {
            let materials = sqlx::query!("
                    SELECT
                        material_type_id,
                        quantity
                    FROM item_reprocessing
                    WHERE type_id = $1
                ",
                    *item.type_id,
                )
                .fetch_all(pool)
                .await;

            let materials = if let Ok(x) = materials {
                if x.is_empty() {
                    // add it back to the list
                    total_reprocessed
                        .entry(item.type_id.into())
                        .and_modify(|value: &mut i64| *value += item.quantity)
                        .or_insert(item.quantity);
                }

                x
            } else {
                // add it back to the list
                total_reprocessed
                    .entry(item.type_id.into())
                    .and_modify(|value: &mut i64| *value += item.quantity)
                    .or_insert(item.quantity);

                Vec::new()
            };

            for material in materials {
                let reprocessed = (
                    (
                        item.quantity as f64 * material.quantity as f64
                    ) * (options.scrap_reprocessing.efficiency() / 100f64)
                ).floor() as i64;

                if reprocessed > 0 {
                    total_reprocessed
                        .entry(material.material_type_id.into())
                        .and_modify(|x: &mut i64| *x += reprocessed)
                        .or_insert(reprocessed);
                }
            }
        }
    }

    let mut appraisal_options = AppraisalOptions::default();
    appraisal_options.set_market_id(Some(appraisal.market_id));
    appraisal_options.set_persist(Some(crate::Persistance::NonPersist));

    let appraisal = create_type_ids(
        &pool,
        total_reprocessed,
        Some(appraisal_options)
    ).await?;

    Ok(Some(appraisal))
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ReprocessingOptions {
    #[serde(default = "default_ore_reprocessing_efficiency")]
    pub ore_reprocessing:   OreReprocessingEfficiency,
    #[serde(default = "default_gas_decompression")]
    pub gas_decompression:  GasReprocessingEfficiency,
    #[serde(default = "default_scrap_reprocessing_efficiency")]
    pub scrap_reprocessing: ScrapReprocessingEfficiency,
}

fn default_ore_reprocessing_efficiency() -> OreReprocessingEfficiency {
    OreReprocessingEfficiency::NsTataraT2
}

fn default_gas_decompression() -> GasReprocessingEfficiency {
    GasReprocessingEfficiency::TataraLvl5
}

fn default_scrap_reprocessing_efficiency() -> ScrapReprocessingEfficiency {
    ScrapReprocessingEfficiency::Lvl5
}
