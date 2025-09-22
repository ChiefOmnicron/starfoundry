use serde::Deserialize;
use starfoundry_libs_types::RegionId;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::time::Instant;

use crate::{FOLDER_INPUT, Error};

pub fn parse(
    directory: &str,
) -> Result<HashMap<RegionId, Region>, Error> {
    tracing::info!("Parsing mapRegions.yaml");
    let start = Instant::now();

    let path = format!(
        "{}/{}/mapRegions.yaml",
        directory,
        FOLDER_INPUT,
    );

    if !Path::new(&path).exists() {
        return Err(Error::FileDoesNotExist(path));
    }

    let file = File::open(&path)
        .map_err(|x| Error::CannotOpenTypeIdsFile(x, path))?;

    let parsed: HashMap<RegionId, RegionWrapper> = serde_yaml::from_reader(file)
        .map(|x| {
            tracing::info!(
                "Finished parsing mapRegions.yaml, task took {:.2}s",
                start.elapsed().as_secs_f64()
            );
            x
        })
        .map_err(Error::ParseTypeIds)?;
    let parsed = parsed
        .into_iter()
        .map(|(region_id, wrapper)| (region_id, Region {
            name:      wrapper.name.get("en").cloned().unwrap_or_default(),
            region_id: region_id,
        }))
        .collect::<HashMap<_, _>>();
    Ok(parsed)
}

/// Represents a single entry in the yaml for a type
#[derive(Clone, Debug, Deserialize)]
pub struct RegionWrapper {
    /// Name of the region
    pub name: HashMap<String, String>,
}

#[derive(Clone, Debug)]
pub struct Region {
    pub region_id: RegionId,
    pub name:      String,
}
