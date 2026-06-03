pub mod blueprints_dependencies;
pub mod blueprints_json;
pub mod blueprints;
pub mod dogma;
pub mod downloads;
pub mod items;
pub mod map;
pub mod parser;
pub mod reprocessing;
pub mod structure;
pub mod systems;

mod error;
use crate::parser::stars::Star;
use crate::parser::systems::{Position, Position2d, System};

pub use self::error::*;

use serde::Serialize;
use sqlx::PgPool;
use starfoundry_lib_types::{RegionId, SystemId};
use std::collections::HashMap;
use std::fs;

// Folder that contains the input file
pub const FOLDER_INPUT: &str  = "input";

pub async fn import_sde(
    pool:               &PgPool,
    current_checksum:   Option<String>,
) -> Result<String, Error> {
    let current_dir = std::env::current_dir().map_err(Error::IoError)?;
    let directory = current_dir.to_str().unwrap_or_default();
    let _ = fs::create_dir(format!("{directory}/{FOLDER_INPUT}"));

    let checksum = downloads::checksum(directory).await?;

    if current_checksum == Some(checksum.clone()) {
        return Ok(checksum);
    }

    downloads::download_assets(directory).await?;

    let categories                = parser::categories::parse(&directory)?;
    let constellations            = parser::constellations::parse(&directory)?;
    let dogma_effects             = parser::dogma_effects::parse(&directory)?;
    let group_ids                 = parser::groups::parse(&directory)?;
    let industry_modifier_sources = parser::industry_modifier_sources::parse(&directory)?;
    let industry_target_filters   = parser::industry_target_filters::parse(&directory)?;
    let regions                   = parser::regions::parse(&directory)?;
    let repackaged                = parser::repackaged::parse(&directory)?;
    let stars                     = parser::stars::parse(&directory)?;
    let systems                   = parser::systems::parse(&directory)?;
    let type_dogma                = parser::type_dogma::parse(&directory)?;
    let type_material             = parser::type_material::parse(&directory)?;
    let asteroid_belts            = parser::asteroid_belt::parse(&directory)?;
    let npc_stations              = parser::npc_station::parse(&directory)?;
    let moons                     = parser::moon::parse(&directory)?;
    let planets                   = parser::planet::parse(&directory)?;
    let stargates                 = parser::stargate::parse(&directory)?;
    let overwrites                = parser::overwrite::parse(&directory)?;

    let mut blueprints            = parser::blueprints::parse(&directory)?;
    let mut type_ids              = parser::type_ids::parse(&directory)?;

    blueprints.extend(overwrites.blueprints);
    type_ids.extend(overwrites.items);

    blueprints_dependencies::run(
            &pool,
            &blueprints,
            &type_ids
        )
        .await?;
    blueprints_json::run(
            &pool,
            &blueprints,
            &categories,
            &group_ids,
            &type_ids,
            &repackaged,
        )
        .await?;
    blueprints::run(
            &pool,
            &blueprints,
            &type_ids,
        )
        .await?;
    dogma::run(
            &pool,
            &dogma_effects,
            &industry_modifier_sources,
            &industry_target_filters,
            &type_dogma,
        )
        .await?;
    items::run(
            &pool,
            &categories,
            &group_ids,
            &type_ids,
            &repackaged,
        )
        .await?;
    reprocessing::run(
            &pool,
            &type_material,
        )
        .await?;
    structure::run(
            &pool,
            &type_ids,
            &type_dogma,
        )
        .await?;
    systems::run(
            &pool,
            regions,
            constellations,
            systems,
        )
        .await?;

    //map::full_map(systems, stargates, regions);

    Ok(checksum)
}
