mod blueprints_dependencies;
mod blueprints_json;
mod blueprints;
mod dogma;
mod downloads;
mod items;
mod parser;
mod reprocessing;
mod structure;
mod systems;

mod error;
use crate::parser::stars::Star;
use crate::parser::systems::{Position, Position2d, System};

pub use self::error::*;

use serde::Serialize;
use sqlx::PgPool;
use starfoundry_lib_types::{RegionId, SystemId};
use std::collections::HashMap;
use std::fs;

/// Folder that contains the input file
pub const FOLDER_INPUT: &str  = "input";

pub async fn import_sde(
    pool: &PgPool,
    current_checksum: Option<String>,
) -> Result<String, Error> {
    let current_dir = std::env::current_dir().map_err(Error::IoError)?;
    let directory = current_dir.to_str().unwrap_or_default();
    let _ = fs::create_dir(format!("{directory}/{FOLDER_INPUT}"));

    let checksum = downloads::checksum(directory).await?;

    if current_checksum == Some(checksum.clone()) {
        return Ok(checksum);
    }

    downloads::download_assets(directory).await?;

    let blueprints                = parser::blueprints::parse(&directory)?;
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
    let type_ids                  = parser::type_ids::parse(&directory)?;
    let type_material             = parser::type_material::parse(&directory)?;

    write_system_json(systems.clone(), stars);

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

    Ok(checksum)
}

// https://github.com/edutechtammy/star-color-and-temperature/blob/main/script.js
fn write_system_json(
    systems: Vec<System>,
    stars:   Vec<Star>,
) {
    let stars = stars
        .into_iter()
        .map(|x| (x.star_id, x))
        .collect::<HashMap<_, _>>();

    #[derive(Debug, Serialize)]
    struct TmpSystem {
        system_id:   SystemId,
        region_id:   RegionId,
        position:    Position,
        #[serde(rename = "position2D")]
        position_2d: Option<Position2d>,
        star:        Star,
    }

    let systems = systems
        .into_iter()
        .filter(|x| x.star_id.is_some())
        .filter(|x|
            x.region_id != RegionId(10000004) &&
            x.region_id != RegionId(10000017) &&
            x.region_id != RegionId(10000019) &&
            (*x.region_id) < 11000000
        )
        .filter(|x| x.position_2d.is_some())
        .map(|x| TmpSystem {
            position:    x.position,
            position_2d: x.position_2d,
            region_id:   x.region_id,
            system_id:   x.system_id,
            star:        stars.get(&x.star_id.unwrap()).unwrap().clone(),
        })
        .collect::<Vec<_>>();

    let mut file = std::fs::File::create("../webapp_components/loading_animation/map.json").unwrap();
    serde_json::to_writer(&mut file, &systems).unwrap();
}
