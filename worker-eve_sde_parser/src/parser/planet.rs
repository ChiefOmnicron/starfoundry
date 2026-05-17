use serde::{Deserialize, Serialize};
use starfoundry_lib_types::{OrbitId, SystemId, TypeId};
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::time::Instant;

use crate::{FOLDER_INPUT, Error};
use crate::parser::systems::Position;

pub fn parse(
    directory: &str,
) -> Result<Vec<Planet>, Error> {
    tracing::info!("Parsing mapPlanets.yaml");
    let start = Instant::now();

    let path = format!(
        "{}/{}/mapPlanets.yaml",
        directory,
        FOLDER_INPUT,
    );

    if !Path::new(&path).exists() {
        return Err(Error::FileDoesNotExist(path));
    }

    let file = File::open(&path)
        .map_err(|x| Error::CannotOpenTypeIdsFile(x, path))?;

    let parsed: HashMap<OrbitId, PlanetWrapper> = serde_yaml::from_reader(file)
        .map(|x| {
            tracing::info!(
                "Finished parsing mapPlanets.yaml, task took {:.2}s",
                start.elapsed().as_secs_f64()
            );
            x
        })
        .map_err(Error::ParseTypeIds)?;
    let parsed = parsed
        .into_iter()
        .map(|(orbit_id, wrapper)| Planet {
            planet_id:          orbit_id,
            asteroid_belt_ids:  wrapper.asteroid_belt_ids,
            moon_ids:           wrapper.moon_ids,
            orbit_id:           wrapper.orbit_id,
            system_id:          wrapper.system_id,
            type_id:            wrapper.type_id,
            position:           wrapper.position,
            radius:             wrapper.radius,
        })
        .collect::<Vec<_>>();
    Ok(parsed)
}

/// Represents a single entry in the yaml for a type
#[derive(Clone, Debug, Deserialize)]
pub struct PlanetWrapper {
    #[serde(rename = "asteroidBeltIDs")]
    #[serde(default)]
    pub asteroid_belt_ids:  Vec<OrbitId>,
    #[serde(rename = "moonIDs")]
    #[serde(default)]
    pub moon_ids:           Vec<OrbitId>,
    #[serde(rename = "orbitID")]
    pub orbit_id:           OrbitId,
    #[serde(rename = "solarSystemID")]
    pub system_id:          SystemId,
    #[serde(rename = "typeID")]
    pub type_id:            TypeId,
    pub position:           Position,
    pub radius:             f64,
}

#[derive(Clone, Debug, Serialize)]
pub struct Planet {
    pub planet_id:          OrbitId,
    pub asteroid_belt_ids:  Vec<OrbitId>,
    pub moon_ids:           Vec<OrbitId>,
    pub orbit_id:           OrbitId,
    pub system_id:          SystemId,
    pub type_id:            TypeId,
    pub position:           Position,
    pub radius:             f64,
}

