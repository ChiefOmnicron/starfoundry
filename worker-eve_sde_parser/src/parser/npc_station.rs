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
) -> Result<Vec<NpcStation>, Error> {
    tracing::info!("Parsing npcStations.yaml");
    let start = Instant::now();

    let path = format!(
        "{}/{}/npcStations.yaml",
        directory,
        FOLDER_INPUT,
    );

    if !Path::new(&path).exists() {
        return Err(Error::FileDoesNotExist(path));
    }

    let file = File::open(&path)
        .map_err(|x| Error::CannotOpenTypeIdsFile(x, path))?;

    let parsed: HashMap<OrbitId, NpcStationWrapper> = serde_yaml::from_reader(file)
        .map(|x| {
            tracing::info!(
                "Finished parsing npcStations.yaml, task took {:.2}s",
                start.elapsed().as_secs_f64()
            );
            x
        })
        .map_err(Error::ParseTypeIds)?;
    let parsed = parsed
        .into_iter()
        .map(|(orbit_id, wrapper)| NpcStation {
            npc_station_id:     orbit_id,
            system_id:          wrapper.system_id,
            type_id:            wrapper.type_id,
            orbit_id:           wrapper.orbit_id,
            orbit_index:        wrapper.orbit_index,
            celestial_index:    wrapper.celestial_index,
            position:           wrapper.position,
            owner_id:           wrapper.owner_id,
        })
        .collect::<Vec<_>>();
    Ok(parsed)
}

/// Represents a single entry in the yaml for a type
#[derive(Clone, Debug, Deserialize)]
pub struct NpcStationWrapper {
    #[serde(rename = "solarSystemID")]
    pub system_id:          SystemId,
    #[serde(rename = "typeID")]
    pub type_id:            TypeId,
    #[serde(rename = "ownerID")]
    pub owner_id:           TypeId,
    #[serde(rename = "orbitID")]
    pub orbit_id:           OrbitId,
    #[serde(rename = "orbitIndex")]
    pub orbit_index:        Option<i32>,
    #[serde(rename = "celestialIndex")]
    pub celestial_index:    Option<i32>,
    pub position:           Position,
}

#[derive(Clone, Debug, Serialize)]
pub struct NpcStation {
    pub npc_station_id:     OrbitId,
    pub system_id:          SystemId,
    pub type_id:            TypeId,
    pub owner_id:           TypeId,
    pub orbit_id:           OrbitId,
    pub orbit_index:        Option<i32>,
    pub celestial_index:    Option<i32>,
    pub position:           Position,
}

