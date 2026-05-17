use serde::Deserialize;
use starfoundry_lib_types::{ConstellationId, RegionId, SystemId};
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::time::Instant;

use crate::{FOLDER_INPUT, Error};
use crate::parser::systems::Position;

pub fn parse(
    directory: &str,
) -> Result<Vec<Constellation>, Error> {
    tracing::info!("Parsing mapConstellations.yaml");
    let start = Instant::now();

    let path = format!(
        "{}/{}/mapConstellations.yaml",
        directory,
        FOLDER_INPUT,
    );

    if !Path::new(&path).exists() {
        return Err(Error::FileDoesNotExist(path));
    }

    let file = File::open(&path)
        .map_err(|x| Error::CannotOpenTypeIdsFile(x, path))?;

    let parsed: HashMap<ConstellationId, ConstellationWrapper> = serde_yaml::from_reader(file)
        .map(|x| {
            tracing::info!(
                "Finished parsing mapConstellations.yaml, task took {:.2}s",
                start.elapsed().as_secs_f64()
            );
            x
        })
        .map_err(Error::ParseTypeIds)?;
    let parsed = parsed
        .into_iter()
        .map(|(constellation_id, wrapper)| Constellation {
            constellation_id:   constellation_id,
            name:               wrapper.name.get("en").cloned().unwrap_or_default(),
            region_id:          wrapper.region_id,
            system_id:          wrapper.system_id,
            position:           wrapper.position,
        })
        .collect::<Vec<_>>();
    Ok(parsed)
}

/// Represents a single entry in the yaml for a type
#[derive(Clone, Debug, Deserialize)]
pub struct ConstellationWrapper {
    #[serde(rename = "regionID")]
    pub region_id:  RegionId,
    #[serde(rename = "solarSystemIDs")]
    pub system_id:  Vec<SystemId>,
    /// Name of the region
    pub name:       HashMap<String, String>,
    pub position:   Position,
}

#[derive(Clone, Debug)]
pub struct Constellation {
    pub constellation_id:   ConstellationId,
    pub name:               String,
    pub region_id:          RegionId,
    pub system_id:          Vec<SystemId>,
    pub position:           Position,
}
