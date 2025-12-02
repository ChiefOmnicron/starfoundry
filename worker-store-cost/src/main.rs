// Avoid musl's default allocator due to lackluster performance
// https://nickb.dev/blog/default-musl-allocator-considered-harmful-to-performance
#[cfg(target_env = "musl")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

mod appraisal;
mod config;
mod engine;
mod market;
mod structure;
mod system_index;

use self::appraisal::*;

use serde::Serialize;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use starfoundry_lib_items::{load_items, parse, Item};
use starfoundry_lib_structures::StructureGroupUuid;
use starfoundry_lib_types::{SystemId, TypeId};
use std::collections::{HashMap, HashSet};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::time::Duration;
use tracing_subscriber::EnvFilter;
use uuid::Uuid;

use crate::config::{load, Build};
use crate::engine::{BlueprintBonus, BlueprintTyp};
use crate::engine::{ProjectConfigBuilder, ProjectStructureGroup, StockMinimal};
use crate::engine::CalculationEngine;
use crate::engine::Dependency;
use crate::market::{material_cost, viable_markets};
use crate::structure::structure_groups;
use crate::system_index::system_index;

#[derive(Debug, Serialize)]
struct StoreContent {
    name: String,
    type_id: TypeId,
    quantity: i64,
}

#[derive(Clone)]
struct NameId {
    name: String,
    uuid: Uuid,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    if cfg!(debug_assertions) {
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .init();
    } else {
        tracing_subscriber::fmt()
            .json()
            .with_env_filter(EnvFilter::from_default_env())
            .init();
    }

    let postgres_industry = PgPoolOptions::new()
        .acquire_timeout(Duration::from_secs(30))
        .connect(&std::env::var("STARFOUNDRY_INDUSTRY_DATABASE_URL").unwrap())
        .await?;
    let postgres_store = PgPoolOptions::new()
        .acquire_timeout(Duration::from_secs(30))
        .connect(&std::env::var("STARFOUNDRY_STORE_DATABASE_URL").unwrap())
        .await?;

    if Path::new("prices.out").exists() {
        std::fs::remove_file("prices.out").unwrap();
    }

    let pwd = std::env::current_dir().unwrap_or(".".into());
    let fits = load(format!("{}/fits", pwd.display()).into())?;
    let hulls = load(format!("{}/hulls", pwd.display()).into())?;

    let mut ids = HashSet::new();
    let mut found_duplicate = false;
    for build in fits.iter() {
        if !ids.insert(build.id) {
            tracing::error!("Duplicate UUID in file {}", build.name);
            found_duplicate = true;
        }
    }

    if found_duplicate {
        panic!("Duplicates");
    }

    let structure_group_ids_hulls: Vec<StructureGroupUuid> = hulls
        .iter()
        .map(|x| x.structure_group.clone())
        .map(Into::into)
        .collect::<Vec<_>>();

    let mut structure_group_ids = fits
        .iter()
        .map(|x| x.structure_group.clone())
        .map(Into::into)
        .collect::<Vec<_>>();
    structure_group_ids.extend(structure_group_ids_hulls);
    structure_group_ids.sort();
    structure_group_ids.dedup();
    let structure_groups = structure_groups(&postgres_industry, structure_group_ids).await?;
    tracing::info!("Loaded structure groups");

    let system_index = system_index(&postgres_industry, structure_groups.clone()).await?;
    tracing::info!("Loaded system index");

    let material_cost = material_cost(&postgres_industry).await?;
    tracing::info!("Loaded material cost");

    let item_cache = load_items(&postgres_industry).await?;

    let mut max_runs: HashMap<TypeId, u32> = HashMap::new();
    max_runs.insert(21009.into(), 40);
    max_runs.insert(21011.into(), 40);
    max_runs.insert(21013.into(), 40);
    max_runs.insert(21017.into(), 40);
    max_runs.insert(21019.into(), 40);
    max_runs.insert(21021.into(), 40);
    max_runs.insert(21023.into(), 40);
    max_runs.insert(21025.into(), 40);
    max_runs.insert(21027.into(), 40);
    max_runs.insert(21029.into(), 40);
    max_runs.insert(21035.into(), 40);
    max_runs.insert(21037.into(), 40);
    max_runs.insert(24545.into(), 40);
    max_runs.insert(24545.into(), 40);
    max_runs.insert(24547.into(), 40);
    max_runs.insert(24556.into(), 40);
    max_runs.insert(24558.into(), 40);
    max_runs.insert(24560.into(), 40);
    max_runs.insert(57474.into(), 40);
    max_runs.insert(57479.into(), 40);
    max_runs.insert(57487.into(), 40);
    max_runs.insert(57486.into(), 200);
    max_runs.insert(57478.into(), 200);
    max_runs.insert(57470.into(), 200);

    let start = std::time::Instant::now();
    let mut build_queue = Vec::new();
    let mut fit_mapping = Vec::new();
    //let mut blueprints = HashSet::new();
    for build in fits {
        if build.id.get_version_num() != 7 {
            panic!("Invalid uuid version {}", build.name);
        }

        fit_mapping.push(NameId {
            name: build.name.clone(),
            uuid: build.id.clone(),
        });

        build_queue.push(process_build(
            postgres_industry.clone(),
            postgres_store.clone(),
            build,
            item_cache.clone(),
            structure_groups.clone(),
            max_runs.clone(),
            system_index.clone(),
            material_cost.clone(),
            Vec::new(),
        ));

        //process_build(
        //    postgres_industry.clone(),
        //    postgres_store.clone(),
        //    build,
        //    item_cache.clone(),
        //    structure_groups.clone(),
        //    max_runs.clone(),
        //    system_index.clone(),
        //    material_cost.clone(),
        //    Vec::new(),
        //).await?;
    }
    let results = futures_util::future::join_all(build_queue).await;
    for result in results {
        if let Err(e) = result {
            tracing::error!(e);
        }
    }
    tracing::info!("Done fits, time: {}", start.elapsed().as_secs());

    tracing::info!("Starting hulls");
    let start = std::time::Instant::now();
    let mut build_queue = Vec::new();
    for build in hulls {
        if build.id.get_version_num() != 7 {
            panic!("Invalid uuid version {}", build.name);
        }

        build_queue.push(process_build(
            postgres_industry.clone(),
            postgres_store.clone(),
            build.clone(),
            item_cache.clone(),
            structure_groups.clone(),
            max_runs.clone(),
            system_index.clone(),
            material_cost.clone(),
            fit_mapping.clone(),
        ));
    }
    let results = futures_util::future::join_all(build_queue).await;
    for result in results {
        if let Err(e) = result {
            tracing::error!(e);
        }
    }
    tracing::info!("Done hulls: {}", start.elapsed().as_secs());

    Ok(())
}

async fn process_build(
    postgres_industry: PgPool,
    postgres_store: PgPool,
    build: Build,
    item_cache: HashMap<String, Item>,
    structure_groups: Vec<ProjectStructureGroup>,
    max_runs: HashMap<TypeId, u32>,
    system_index: HashMap<SystemId, (f32, f32)>,
    material_cost: HashMap<TypeId, f64>,
    fit_mapping: Vec<NameId>,
) -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!("Starting {}", build.name);
    let structure_group = structure_groups
        .iter()
        .find(|x| x.id == build.structure_group)
        .unwrap();

    if structure_group.structures.is_empty() || structure_group.mapping.is_empty() {
        panic!("No structure");
    }

    let products = parse(&item_cache, &build.project);

    let mut blueprint_overwrites = products
        .items
        .iter()
        .map(|x| (x.type_id, BlueprintBonus {
                ptype_id: x.type_id,
                material: x.material_efficiency.unwrap_or_default() as f32,
                time: 0f32,
            })
        )
        .collect::<HashMap<_, _>>();
    blueprint_overwrites.insert(20185.into(), BlueprintBonus { ptype_id: 20185.into(), material: 8f32, time: 0f32 }); // charon
    blueprint_overwrites.insert(20183.into(), BlueprintBonus { ptype_id: 20183.into(), material: 8f32, time: 0f32 }); // providence
    blueprint_overwrites.insert(20187.into(), BlueprintBonus { ptype_id: 20187.into(), material: 8f32, time: 0f32 }); // obelisk
    blueprint_overwrites.insert(20189.into(), BlueprintBonus { ptype_id: 20189.into(), material: 8f32, time: 0f32 }); // fenrir

    // for t2 dreads
    blueprint_overwrites.insert(19726.into(), BlueprintBonus { ptype_id: 19726.into(), material: 8f32, time: 0f32 }); // phoenix
    blueprint_overwrites.insert(19720.into(), BlueprintBonus { ptype_id: 19720.into(), material: 8f32, time: 0f32 }); // revelation
    blueprint_overwrites.insert(19724.into(), BlueprintBonus { ptype_id: 19724.into(), material: 8f32, time: 0f32 }); // moros
    blueprint_overwrites.insert(19722.into(), BlueprintBonus { ptype_id: 19722.into(), material: 8f32, time: 0f32 }); // naglfar

    let config = ProjectConfigBuilder::default()
        .add_blueprint_overwrites(blueprint_overwrites)
        .add_structures(structure_group.structures.clone())
        .add_structure_mappings(structure_group.mapping.clone())
        .set_material_cost(material_cost.clone())
        .set_system_index(system_index.clone())
        .set_max_runs(max_runs.clone())
        .build();

    let mut dependency_tree = CalculationEngine::new(config);

    for module in products.items {
        let meta_molecular_stock = match module.item_name.as_ref() {
            "Apostle" => 9,
            "Archon" => 9,
            "Caiman" => 9,
            "Chemosh" => 9,
            "Chimera" => 9,
            "Dagon" => 9,
            "Lif" => 9,
            "Loggerhead" => 9,
            "Minokawa" => 9,
            "Moros Navy Issue" => 9,
            "Moros" => 9,
            "Naglfar Fleet Issue" => 9,
            "Naglfar" => 9,
            "Nidhoggur" => 9,
            "Ninauzu" => 9,
            "Phoenix Navy Issue" => 9,
            "Phoenix" => 9,
            "Revelation Navy Issue" => 9,
            "Revelation" => 9,
            "Sarathiel" => 9,
            "Thanatos" => 9,
            _ => 0,
        };

        let json = sqlx::query!("
                SELECT data
                FROM   blueprint_json
                WHERE  ptype_id = $1
            ",
                *module.type_id,
            )
            .fetch_one(&postgres_industry)
            .await
            .map(|x| x.data);

        if let Err(e) = &json {
            tracing::error!("[{}] Blueprint Fetch error, {}, {}", build.name, e, module.type_id);
        }

        let json = json.unwrap();
        let json = serde_json::to_value(&json).unwrap();

        if let Ok(x) = Dependency::try_from(module.quantity as u32, json) {
            dependency_tree.add(x);

            if meta_molecular_stock > 0 {
                dependency_tree.add_stocks(&vec![StockMinimal {
                    quantity: meta_molecular_stock,
                    type_id: 57458.into(),
                }]);
            }
        } else {
            continue;
        };
    }
    tracing::info!("[{}] Dependency tree done", build.name);

    let dependency_result = dependency_tree
        .apply_bonus()
        .finalize();
    tracing::info!("[{}] Finished Tree", build.name);

    let market_data = build
        .market
        .lines()
        .collect::<Vec<_>>()
        .join("\n");
    let market_data_response = reqwest::Client::new()
        .post("https://api.appraisal.starfoundry.space/appraisals")
        .header("user-agent", "StarFoundry worker-buildcost (0.0.0)")
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "appraisal": market_data,
            "market": 60003760,
            "persist": "NonPersist",
            "price_modifier": 100
        }))
        .send()
        .await?
        .json::<Appraisal>()
        .await;
    let market_data_response = match market_data_response {
        Ok(x) => x,
        Err(e) => {
            dbg!(&e, build.store);
            panic!("Appraisal Market response")
        }
    };
    tracing::info!("[{}] Fetched market data", build.name);

    let store_content_response = reqwest::Client::new()
        .post("https://api.appraisal.starfoundry.space/appraisals")
        .header("user-agent", "StarFoundry worker-buildcost (0.0.0)")
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "appraisal": build.store,
            "market": 60003760,
            "persist": "NonPersist",
            "price_modifier": 100
        }))
        .send()
        .await?
        .json::<Appraisal>()
        .await;
    let store_content_response = match store_content_response {
        Ok(x) => x,
        Err(e) => {
            dbg!(&e, build.store);
            panic!("Appraisal Store response")
        }
    };

    let store_content_response = store_content_response
        .items
        .into_iter()
        .map(|x| StoreContent {
            name: x.meta.name,
            quantity: x.quantity,
            type_id: x.type_id
        })
        .collect::<Vec<_>>();
    let store_content_response = serde_json::to_value(&store_content_response)?;
    tracing::info!("[{}] Fetched store data", build.name);

    let manufacturing_cost: f32 = dependency_result
        .tree
        .iter()
        .filter(|(_, x)|
            x.typ == BlueprintTyp::Blueprint ||
            x.typ == BlueprintTyp::Reaction
        )
        .map(|(_, x)| x.build_cost.total_job_cost)
        .sum();
    tracing::info!("[{}] Collected manufacturing cost - amount: {}", build.name, manufacturing_cost);

    let jita_market = viable_markets(
        &postgres_industry,
        dependency_result.tree.clone(),
        vec![
            60003760i64,
            60008494i64,
            1046664001931i64,
        ]
    )
    .await?;

    let market_cost: f32 = jita_market
        .iter()
        .map(|(_, x)| x.price as f32 * x.quantity as f32)
        .sum();
    tracing::info!("[{}] Collected market cost", build.name);

    let bpc_cost = build.bpc.iter().map(|x| x.price as f32).sum();

    let market_result = MarketResult {
        manufacturing: manufacturing_cost,
        manufacturing_market: market_cost,
        market: market_data_response.total.sell as f32,
        bpc: bpc_cost,
        name: build.name.clone(),
    };

    tracing::info!(
        "[{}] BPC: {} - Market: {} - Manufacturing: {}, Market: {} - Total: {}",
        market_result.name,
        market_result.bpc,
        market_result.market,
        market_result.manufacturing,
        market_result.manufacturing_market,
        market_result.market + market_result.manufacturing + market_result.manufacturing_market + market_result.bpc,
    );
    let price_string = format!(
        "[{:30}]\tBPC\t{:15}\tMarket\t{:15}\tManufacturing Job cost\t{:15}\tManufacturing market Cost\t{:15}\tTotal\t{:15}",
        market_result.name,
        market_result.bpc,
        market_result.market,
        market_result.manufacturing,
        market_result.manufacturing_market,
        market_result.market + market_result.manufacturing + market_result.manufacturing_market + market_result.bpc,
    );

    let mut additional_products = fit_mapping
        .iter()
        .filter(|x| x.name.starts_with(&format!("[{}]", build.name)))
        .map(|x| x.uuid)
        .collect::<Vec<_>>();
    additional_products.sort();
    additional_products.dedup();

    let mut transaction = postgres_store.begin().await?;
    sqlx::query!("
            INSERT INTO product (
                price,
                id,
                category,
                name,
                image_type,
                image_type_id,
                content,
                tags,
                message,
                delivery_time,
                blacklist,
                whitelist,
                additional_products,
                delivery_location,
                hidden
            )
            VALUES (1, $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
            ON CONFLICT (id)
            DO UPDATE SET
                category = EXCLUDED.category,
                name = EXCLUDED.name,
                image_type = EXCLUDED.image_type,
                image_type_id = EXCLUDED.image_type_id,
                content = EXCLUDED.content,
                tags = EXCLUDED.tags,
                message = EXCLUDED.message,
                delivery_time = EXCLUDED.delivery_time,
                blacklist = EXCLUDED.blacklist,
                whitelist = EXCLUDED.whitelist,
                additional_products = EXCLUDED.additional_products,
                delivery_location = EXCLUDED.delivery_location,
                hidden = EXCLUDED.hidden
        ",
            build.id,
            build.category,
            build.name,
            build.image_type,
            build.image_type_id,
            &store_content_response,
            &build.tags,
            build.message.as_ref(),
            build.delivery_time,
            &build.blacklist.unwrap_or_default().iter().map(|x| **x).collect::<Vec<_>>(),
            &build.whitelist.unwrap_or_default().iter().map(|x| **x).collect::<Vec<_>>(),
            &additional_products,
            &build.delivery_location,
            &build.hidden.unwrap_or_default(),
        )
        .execute(&mut *transaction)
        .await?;
    tracing::info!("[{}] Inserted products", build.name);

    sqlx::query!("
            DELETE FROM buildcost_history
            WHERE date = NOW()
        ")
        .execute(&mut *transaction)
        .await?;

    let multiplier = match build.image_type_id {
        // Avatar
        11567 |
        // Azariel
        78576 |
        // Erebus
        671 |
        // Ragnarok
        23773 |
        // Leviathan
        3764 |
        // Aeon
        23919 |
        // Hel
        22852 |
        // Nyx
        23913 |
        // Komodo
        45649 |
        // Molok
        42241 |
        // Revenant
        3514 |
        // Vendetta
        42125 |
        // Wyvern
        23917 |
        // Ark
        28850 |
        // Anshar
        28848 |
        // Nomad
        28846 |
        // Rhea
        28844 => 1.15,
        _ => 1.1,
    };

    let total = market_result.manufacturing + market_result.manufacturing_market;
    let sell_price = (total * multiplier) + market_result.market + market_result.bpc;
    let sell_price = if sell_price > 10_000_000_000f32 {
        ((sell_price / 1_000_000_000f32).ceil() as i64) * 1_000_000_000i64
    } else if sell_price > 1_000_000_000f32 {
        ((sell_price / 100_000_000f32).ceil() as i64) * 100_000_000i64
    } else {
        ((sell_price / 10_000_000f32).ceil() as i64) * 10_000_000i64
    };

    sqlx::query!("
            INSERT INTO buildcost_history (
                product_id,
                bpc,
                market,
                manufacturing,
                manufacturing_market,
                total,
                sell_price
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
        ",
            build.id,
            market_result.bpc,
            market_result.market,
            market_result.manufacturing,
            market_result.manufacturing_market,
            market_result.market + market_result.manufacturing + market_result.manufacturing_market + market_result.bpc,
            sell_price as i64,
        )
        .execute(&mut *transaction)
        .await?;
    tracing::info!("[{}] Inserted history", build.name);

    dbg!(sell_price);
    sqlx::query!("
            UPDATE product
            SET price = $2
            WHERE id = $1
        ",
            build.id,
            sell_price as i64,
        )
        .execute(&mut *transaction)
        .await?;

    transaction.commit().await?;
    tracing::info!("[{}] Done {}", build.name, build.name);

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("prices.out")
        .unwrap();
    writeln!(file, "{}", price_string).unwrap();

    Ok(())
}

#[derive(Debug)]
struct MarketResult {
    bpc: f32,
    manufacturing_market: f32,
    manufacturing: f32,
    market: f32,
    name: String,
}
