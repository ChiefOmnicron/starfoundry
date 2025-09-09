use serde::Deserialize;
use starfoundry_lib_types::{ConstellationId, RegionId, SystemId};
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::time::Instant;

use crate::{FOLDER_INPUT, Error};

pub fn parse(
    directory: &str,
) -> Result<Vec<System>, Error> {
    tracing::info!("Parsing mapSolarSystems.yaml");
    let start = Instant::now();

    let path = format!(
        "{}/{}/mapSolarSystems.yaml",
        directory,
        FOLDER_INPUT,
    );

    if !Path::new(&path).exists() {
        return Err(Error::FileDoesNotExist(path));
    }

    let file = File::open(&path)
        .map_err(|x| Error::CannotOpenTypeIdsFile(x, path))?;

    let parsed: HashMap<SystemId, SystemWrapper> = serde_yaml::from_reader(file)
        .map(|x| {
            tracing::info!(
                "Finished parsing mapSolarSystems.yaml, task took {:.2}s",
                start.elapsed().as_secs_f64()
            );
            x
        })
        .map_err(Error::ParseTypeIds)?;
    let parsed = parsed
        .into_iter()
        .map(|(system_id, wrapper)| System {
            name:             wrapper.name.get("en").cloned().unwrap_or_default(),
            region_id:        wrapper.region_id,
            constellation_id: wrapper.constellation_id,
            security:         wrapper.security,
            system_id:        system_id,
        })
        .collect::<Vec<_>>();
    Ok(parsed)
}

/// Represents a single entry in the yaml for a type
#[derive(Clone, Debug, Deserialize)]
pub struct SystemWrapper {
    /// Name of the region
    pub name:             HashMap<String, String>,
    #[serde(rename = "regionID")]
    pub region_id:        RegionId,
    #[serde(rename = "constellationID")]
    pub constellation_id: ConstellationId,
    #[serde(rename = "securityStatus")]
    pub security:         f32,
}

#[derive(Clone, Debug)]
pub struct System {
    pub region_id:        RegionId,
    pub constellation_id: ConstellationId,
    pub system_id:        SystemId,
    pub name:             String,
    pub security:         f32,
}
