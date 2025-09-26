use serde::Deserialize;
use sqlx::PgPool;
use starfoundry_libs_types::{RegionId, SystemId};
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::time::Instant;

use crate::parser::regions::Region;
use crate::parser::systems::System;
use crate::FOLDER_INPUT;

pub async fn run(
    pool: &PgPool,
    regions: HashMap<RegionId, Region>,
    systems: Vec<System>,
) -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!("Processing systems");
    let start = Instant::now();

    insert_into_database(
            &pool,
            systems,
            regions,
        )
        .await?;

    tracing::info!(
        "Finished processing systems, task took {:.2}s",
        start.elapsed().as_secs_f64()
    );

    Ok(())
}

async fn insert_into_database(
    pool:       &PgPool,
    systems:    Vec<System>,
    regions:    HashMap<RegionId, Region>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut transaction = pool
        .begin()
        .await?;

    tracing::debug!("Clearing system database");
    sqlx::query!("
            DELETE FROM system
        ")
        .execute(&mut *transaction)
        .await?;
    tracing::debug!("Clearing systems database done");

    let region_ids = systems
        .iter()
        .map(|x| *x.region_id)
        .collect::<Vec<_>>();
    let region_names = systems
        .iter()
        .map(|x| regions.get(&x.region_id).unwrap().name.clone())
        .collect::<Vec<_>>();
    let system_ids = systems
        .iter()
        .map(|x| *x.system_id)
        .collect::<Vec<_>>();
    let system_names = systems
        .iter()
        .map(|x| x.name.clone())
        .collect::<Vec<_>>();
    let security = systems
        .iter()
        .map(|x| x.security)
        .collect::<Vec<_>>();

    tracing::debug!("Inserting data");
    sqlx::query!("
            INSERT INTO system
            (
                region_id,
                region_name,
                system_id,
                system_name,
                security
            )
            SELECT * FROM UNNEST(
                $1::INTEGER[],
                $2::VARCHAR[],
                $3::INTEGER[],
                $4::VARCHAR[],
                $5::REAL[]
            )
        ",
            &region_ids,
            &region_names,
            &system_ids,
            &system_names,
            &security,
        )
        .execute(&mut *transaction)
        .await?;
    transaction.commit().await?;
    tracing::debug!("Inserting data done");

    Ok(())
}

/*
fn parse_folder(path: &Path) -> Result<Vec<Solarsystem>, Box<dyn std::error::Error>> {
    let mut regions = Vec::new();
    for region in path.read_dir().unwrap() {
        if let Ok(entry) = region {
            let path = entry.path();

            if path.is_file() {
                continue;
            }

            let region = parse_region(
                path.as_path(),
            )?;
            regions.extend(region);
        }
    }

    Ok(regions)
}

fn parse_region(path: &Path) -> Result<Vec<Solarsystem>, Box<dyn std::error::Error>> {
    let region_name = path
        .file_name()
        .map(|x| x.to_str().unwrap_or_default())
        .unwrap_or_default();

    let region_name = match region_name {
        "PeriodBasis"        => "Period Basis".into(),
        "PureBlind"          => "Pure Blind".into(),
        "OuterPassage"       => "Outer Passage".into(),
        "ParagonSoul"        => "Paragon Soul".into(),
        "TheSpire"           => "The Spire".into(),
        "TheForge"           => "The Forge".into(),
        "GreatWildlands"     => "Great Wildlands".into(),
        "PerrigenFalls"      => "Perrigen Falls".into(),
        "TheCitadel"         => "The Citadel".into(),
        "WickedCreek"        => "Wicked Creek".into(),
        "ValeoftheSilent"    => "Vale of the Silent".into(),
        "ScaldingPass"       => "Scalding Pass".into(),
        "VergeVendor"        => "Verge Vendor".into(),
        "TheBleakLands"      => "The Bleak Lands".into(),
        "OuterRing"          => "Outer Ring".into(),
        "TheKalevalaExpanse" => "The Kalevala Expanse".into(),
        "SinqLaison"         => "Sinq Laison".into(),
        "MoldenHeath"        => "Molden Heath".into(),
        "BlackRise"          => "Black Rise".into(),
        "CloudRing"          => "Cloud Ring".into(),
        "EtheriumReach"      => "Etherium Reach".into(),
        "CobaltEdge"         => "Cobalt Edge".into(),
        _ => region_name,
    };

    // Jove Empire
    if region_name == "A821-A" {
        return Ok(Vec::new());
    }
    // No clue
    if region_name == "UUA-F4" || region_name == "J7HZ-F" {
        return Ok(Vec::new());
    }
    // Pochven
    if region_name == "Pochven" {
        return Ok(Vec::new());
    }

    let region_path = path.join("region.staticdata");
    let mut region: Region = serde_yaml::from_reader(
        File::open(region_path)?
    )?;
    region.name = region_name.to_string();

    let mut constellations = Vec::new();
    for constellation in path.read_dir().unwrap() {
        if let Ok(entry) = constellation {
            let path = entry.path();

            if path.is_file() {
                continue;
            }

            let constellation = parse_constellation(
                path.as_path(),
                &region,
            )?;
            constellations.extend(constellation);
        }
    }

    Ok(constellations)
}

fn parse_constellation(
    path:   &Path,
    region: &Region,
) -> Result<Vec<Solarsystem>, Box<dyn std::error::Error>> {
    let constellation_name = path
        .file_name()
        .map(|x| x.to_str().unwrap_or_default())
        .unwrap_or_default();

    let constellation_path = path.join("constellation.staticdata");
    let mut constellation: Constellation = serde_yaml::from_reader(
        File::open(constellation_path.clone())?
    )?;
    constellation.name = constellation_name.to_string();

    let mut systems = Vec::new();
    for system in path.read_dir().unwrap() {
        if let Ok(entry) = system {
            let path = entry.path();

            if path.is_file() {
                continue;
            }
            let system = parse_system(
                path.as_path(),
                &region,
                &constellation,
            )?;
            systems.push(system);
        }
    }

    Ok(systems)
}

fn parse_system(
    path:          &Path,
    region:        &Region,
    constellation: &Constellation,
) -> Result<Solarsystem, Box<dyn std::error::Error>> {
    let solarsystem_name = path
        .file_name()
        .map(|x| x.to_str().unwrap_or_default())
        .unwrap_or_default();

    let solarsystem_path = path.join("solarsystem.staticdata");
    let mut solarsystem: Solarsystem = serde_yaml::from_reader(
        File::open(solarsystem_path.clone())?
    )?;

    solarsystem.system_name = solarsystem_name.to_string();

    solarsystem.region_id = region.region_id;
    solarsystem.region_name = region.name.clone();

    solarsystem.constellation_id = constellation.constellation_id;
    solarsystem.constellation_name = constellation.name.clone();

    Ok(solarsystem)
}

#[derive(Debug, Deserialize)]
struct Region {
    #[serde(rename = "regionID")]
    region_id: i32,

    #[serde(skip_deserializing)]
    name: String,
}

#[derive(Debug, Deserialize)]
struct Constellation {
    #[serde(rename = "constellationID")]
    constellation_id: i32,

    #[serde(skip_deserializing)]
    name: String,
}

#[derive(Debug, Deserialize)]
struct Solarsystem {
    #[serde(skip_deserializing)]
    region_id: i32,
    #[serde(skip_deserializing)]
    region_name: String,

    #[serde(skip_deserializing)]
    constellation_id: i32,
    #[serde(skip_deserializing)]
    constellation_name: String,

    #[serde(rename = "solarSystemID")]
    system_id: i32,
    security: f32,

    #[serde(skip_deserializing)]
    system_name: String,
}
*/
