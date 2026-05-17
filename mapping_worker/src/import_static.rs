use sqlx::PgPool;
use starfoundry_lib_eve_sde_parser::{downloads, parser, systems};
use std::fs;

/// Folder that contains the input file
pub const FOLDER_INPUT: &str = "input";

pub async fn import_static(
    pool: &PgPool,
) -> Result<(), Box<dyn std::error::Error>> {
    let current_dir = std::env::current_dir()?;
    let directory = current_dir.to_str().unwrap_or_default();
    let _ = fs::create_dir(format!("{directory}/{FOLDER_INPUT}"));
    downloads::download_assets(directory).await?;

    let systems                   = parser::systems::parse(&directory)?;
    let constellations            = parser::constellations::parse(&directory)?;
    let regions                   = parser::regions::parse(&directory)?;
    let moons                     = parser::moon::parse(&directory)?;
    let planets                   = parser::planet::parse(&directory)?;
    let stargates                 = parser::stargate::parse(&directory)?;
    let stars                     = parser::stars::parse(&directory)?;

    Ok(())
}
