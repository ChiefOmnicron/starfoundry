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
) -> Result<Vec<Moon>, Error> {
    tracing::info!("Parsing mapMoons.yaml");
    let start = Instant::now();

    let path = format!(
        "{}/{}/mapMoons.yaml",
        directory,
        FOLDER_INPUT,
    );

    if !Path::new(&path).exists() {
        return Err(Error::FileDoesNotExist(path));
    }

    let file = File::open(&path)
        .map_err(|x| Error::CannotOpenTypeIdsFile(x, path))?;

    let parsed: HashMap<OrbitId, MoonWrapper> = serde_yaml::from_reader(file)
        .map(|x| {
            tracing::info!(
                "Finished parsing mapMoons.yaml, task took {:.2}s",
                start.elapsed().as_secs_f64()
            );
            x
        })
        .map_err(Error::ParseTypeIds)?;
    let parsed = parsed
        .into_iter()
        .map(|(orbit_id, wrapper)| Moon {
            moon_id:    orbit_id,
            orbit_id:   wrapper.orbit_id,
            system_id:  wrapper.system_id,
            type_id:    wrapper.type_id,
            position:   wrapper.position,
            radius:     wrapper.radius,
        })
        .collect::<Vec<_>>();
    Ok(parsed)
}

/// Represents a single entry in the yaml for a type
#[derive(Clone, Debug, Deserialize)]
pub struct MoonWrapper {
    /// either a planet or a star
    #[serde(rename = "orbitID")]
    pub orbit_id:   OrbitId,
    #[serde(rename = "solarSystemID")]
    pub system_id:  SystemId,
    #[serde(rename = "typeID")]
    pub type_id:    TypeId,
    pub position:   Position,
    pub radius:     f64,
}

#[derive(Clone, Debug, Serialize)]
pub struct Moon {
    /// either a planet or a star
    pub moon_id:    OrbitId,
    pub orbit_id:   OrbitId,
    pub system_id:  SystemId,
    pub type_id:    TypeId,
    pub position:   Position,
    pub radius:     f64,
}

