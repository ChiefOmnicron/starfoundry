use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use starfoundry_libs_compression::{calculate_gas, calculate_ore, overage, Asteroid, Config, Gas, GasReprocessingEfficiency, OreReprocessingEfficiency};
use starfoundry_libs_types::TypeId;
use std::collections::{HashMap, HashSet};
use utoipa::ToSchema;

use crate::internal::{create_type_ids, AppraisalOptions};
use crate::{Error, Result};
use super::Appraisal;

pub async fn compression(
    pool:    &PgPool,
    code:    String,
    options: CompressionOptions,
) -> Result<Option<CompressionResult>> {
    // fetch the appraisal
    let appraisal = if let Some(x) = super::fetch(pool, code).await? {
        x
    } else {
        return Ok(None);
    };

    let mut non_minerals_gas = HashMap::new();
    let mut compression_config = Config::default();
    compression_config.ore_reprocessing        = options.ore_reprocessing;
    compression_config.gas_decompression       = options.gas_decompression;
    compression_config.allow_minerals          = options.allow_minerals;
    compression_config.allow_uncompressed_gas  = options.allow_uncompressed_gas;
    compression_config.allow_uncompressed_moon = options.allow_uncompressed_moon;
    compression_config.allow_uncompressed_ore  = options.allow_uncompressed_ore;

    compression_config.prices_asteroid = fetch_asteroid_prices_average(&pool).await;
    compression_config.prices_gas = fetch_gas_prices_average(&pool).await;
    compression_config.limit_asteroid = fetch_astoid_limits(&pool).await;
    compression_config.limit_gas = fetch_gas_limits(&pool).await;
    compression_config.blacklist = options
        .blacklist
        .into_iter()
        .map(|x| Asteroid::blacklist(*x))
        .flatten()
        .map(|x| Asteroid::from_type_id(x))
        .collect::<HashSet<_>>();

    appraisal
        .items
        .into_iter()
        .for_each(|x| {
            if
                // minerals
                x.meta.group_id == 18.into() ||
                // moon materials
                x.meta.group_id == 427.into() {

                compression_config
                    .want_mineral
                    .entry(x.type_id.into())
                    .and_modify(|value: &mut f64| *value += x.quantity as f64)
                    .or_insert(x.quantity as f64);
            } else if x.meta.group_id == 711.into() {
                // gas
                compression_config
                    .want_gas
                    .entry(x.type_id.into())
                    .and_modify(|value: &mut f64| *value += x.quantity as f64)
                    .or_insert(x.quantity as f64);
            } else {
                non_minerals_gas
                    .entry(x.type_id)
                    .and_modify(|value: &mut i64| *value += x.quantity as i64)
                    .or_insert(x.quantity as i64);
            }
        });

    let compressed_gas = if compression_config.want_gas.is_empty() {
        HashMap::new()
    } else {
        calculate_gas(&compression_config)
            .map_err(|_| Error::NoSolution)?
            .into_iter()
            .map(|(type_id, quantity)| (type_id.into(), quantity.ceil() as i64))
            .collect::<HashMap<_, _>>()
    };

    let compressed_minerals = if compression_config.want_mineral.is_empty() {
        HashMap::new()
    } else {
        calculate_ore(&compression_config)
            .map_err(|_| Error::NoSolution)?
            .into_iter()
            .map(|(type_id, quantity)| (type_id.into(), quantity.ceil() as i64))
            .collect::<HashMap<_, _>>()
    };

    let mut appraisal_options = AppraisalOptions::default();
    appraisal_options.set_market_id(Some(appraisal.market_id));
    appraisal_options.set_store(Some(crate::Persistance::NonPersistent));

    let appraisal_overage = if !compressed_minerals.is_empty() {
        let overage = overage(
                options.ore_reprocessing,
                compressed_minerals.clone(),
                compression_config.want_mineral,
            )
            .into_iter()
            .map(|(mineral, quantity)| (mineral.to_type_id(), quantity as i64))
            .collect::<HashMap<_, _>>();

        let appraisal_overage = create_type_ids(
            &pool,
            overage,
            Some(appraisal_options.clone())
        ).await?;
        Some(appraisal_overage)
    } else {
        None
    };

    let mut items_need = HashMap::new();
    items_need.extend(compressed_minerals);
    items_need.extend(compressed_gas);
    items_need.extend(non_minerals_gas);

    let appraisal_goal = create_type_ids(
        &pool,
        items_need,
        Some(appraisal_options)
    ).await?;

    Ok(Some(CompressionResult {
        compression_appraisal: appraisal_goal,
        overage_appraisal:     appraisal_overage,
    }))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CompressionResult {
    pub compression_appraisal: Appraisal,
    pub overage_appraisal:     Option<Appraisal>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CompressionOptions {
    #[serde(default = "default_ore_reprocessing")]
    pub ore_reprocessing:        OreReprocessingEfficiency,
    #[serde(default = "default_gas_decompression")]
    pub gas_decompression:       GasReprocessingEfficiency,

    #[serde(default)]
    pub allow_minerals:          bool,
    #[serde(default)]
    pub allow_uncompressed_gas:  bool,
    #[serde(default)]
    pub allow_uncompressed_moon: bool,
    #[serde(default)]
    pub allow_uncompressed_ore:  bool,

    #[serde(default)]
    pub blacklist:               Vec<TypeId>,
}

fn default_ore_reprocessing() -> OreReprocessingEfficiency {
    OreReprocessingEfficiency::NsTataraT2
}

fn default_gas_decompression() -> GasReprocessingEfficiency {
    GasReprocessingEfficiency::TataraLvl5
}

async fn fetch_asteroid_prices_average(
    pool: &PgPool,
) -> HashMap<Asteroid, f64> {
    let type_ids = Asteroid::type_ids();
    let mut prices = HashMap::new();

    sqlx::query!(r#"
            SELECT
                MIN(price) as "price!",
                type_id
            FROM   market_orders_latest
            WHERE  market_orders_latest.type_id = ANY($1)
              AND  is_buy = false
              AND  structure_id = 60003760
            GROUP  BY type_id
        "#,
            &type_ids,
        )
        .fetch_all(pool)
        .await
        .unwrap()
        .into_iter()
        .for_each(|x| {
            prices.insert(
                Asteroid::from_type_id(x.type_id),
                x.price,
            );
        });

    prices
}

async fn fetch_gas_prices_average(
    pool: &PgPool,
) -> HashMap<Gas, f64> {
    let type_ids = Gas::type_ids();
    let mut prices = HashMap::new();

    sqlx::query!(r#"
            SELECT
                MIN(price) as "price!",
                type_id
            FROM   market_orders_latest
            WHERE  market_orders_latest.type_id = ANY($1)
              AND  is_buy = false
              AND  structure_id = 60003760
            GROUP  BY type_id
        "#,
            &type_ids,
        )
        .fetch_all(pool)
        .await
        .unwrap()
        .into_iter()
        .for_each(|x| {
            prices.insert(
                Gas::from_type_id(x.type_id),
                x.price,
            );
        });

    prices
}

async fn fetch_astoid_limits(
    pool: &PgPool,
) -> HashMap<Asteroid, f64> {
    let type_ids = Asteroid::type_ids();
    let mut limits = HashMap::new();

    sqlx::query!("
            SELECT
                SUM(remaining) AS total,
                type_id
            FROM market_orders_latest
            WHERE type_id = ANY($1)
            AND structure_id = 60003760
            AND is_buy = false
            GROUP BY type_id
        ",
            &type_ids,
        )
        .fetch_all(pool)
        .await
        .unwrap()
        .into_iter()
        .for_each(|x| {
            limits.insert(
                Asteroid::from_type_id(x.type_id),
                x.total.map(|x| x as f64).unwrap_or(0f64),
            );
        });

    limits
}

async fn fetch_gas_limits(
    pool: &PgPool,
) -> HashMap<Gas, f64> {
    let type_ids = Gas::type_ids();
    let mut limits = HashMap::new();

    sqlx::query!("
            SELECT
                SUM(remaining) AS total,
                type_id
            FROM market_orders_latest
            WHERE type_id = ANY($1)
            AND structure_id = 60003760
            AND is_buy = false
            GROUP BY type_id
        ",
            &type_ids,
        )
        .fetch_all(pool)
        .await
        .unwrap()
        .into_iter()
        .for_each(|x| {
            limits.insert(
                Gas::from_type_id(x.type_id),
                x.total.map(|x| x as f64).unwrap_or(0f64),
            );
        });

    limits
}
