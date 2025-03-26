#![feature(let_chains)]

mod blueprints_dependencies;
mod blueprints_json;
mod blueprints_temp;
mod dogma;
mod downloads;
mod items;
mod parser;
mod reprocessing;
mod structure;
mod systems;

mod error;
pub use self::error::*;

use sqlx::PgPool;
use tokio::join;
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
    let dogma_effects             = parser::dogma_effects::parse(&directory)?;
    let group_ids                 = parser::group_ids::parse(&directory)?;
    let industry_modifier_sources = parser::industry_modifier_sources::parse(&directory)?;
    let industry_target_filters   = parser::industry_target_filters::parse(&directory)?;
    let repackaged                = parser::repackaged::parse(&directory)?;
    let type_dogma                = parser::type_dogma::parse(&directory)?;
    let type_ids                  = parser::type_ids::parse(&directory)?;
    let type_material             = parser::type_material::parse(&directory)?;

    let blueprints_dependencies = blueprints_dependencies::run(
            &pool,
            &blueprints,
            &type_ids
        );
    let blueprints_json = blueprints_json::run(
            &pool,
            &blueprints,
            &group_ids,
            &type_ids,
            &repackaged,
        );
    let blueprints_temp = blueprints_temp::run(
            &pool,
            &blueprints,
        );
    let dogma = dogma::run(
            &pool,
            &dogma_effects,
            &industry_modifier_sources,
            &industry_target_filters,
            &type_dogma,
        );
    let items = items::run(
            &pool,
            &group_ids,
            &type_ids,
            &repackaged,
        );
    let reprocessing = reprocessing::run(
            &pool,
            &type_material,
        );
    let structure = structure::run(
            &pool,
            &type_ids,
            &type_dogma,
        );
    let systems = systems::run(
            &pool,
        );

    // Ignore errors
    let _ = join! {
        blueprints_dependencies,
        blueprints_json,
        blueprints_temp,
        dogma,
        items,
        reprocessing,
        structure,
        systems,
    };

    Ok(checksum)
}
