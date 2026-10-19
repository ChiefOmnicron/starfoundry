use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::time::Instant;

use serde::Deserialize;
use starfoundry_lib_types::{ConstellationId, RegionId};

use crate::parser::systems::Position;
use crate::{EveSdeParserError, FOLDER_INPUT};

pub fn parse(directory: &str) -> Result<Vec<Region>, EveSdeParserError> {
    tracing::info!("Parsing mapRegions.yaml");
    let start = Instant::now();

    let path = format!("{}/{}/mapRegions.yaml", directory, FOLDER_INPUT,);

    if !Path::new(&path).exists() {
        return Err(EveSdeParserError::FileDoesNotExist(path));
    }

    let file = File::open(&path)
        .map_err(|x| EveSdeParserError::CannotOpenTypeIdsFile(x, path))?;

    let parsed: HashMap<RegionId, RegionWrapper> =
        serde_yaml::from_reader(file)
            .map(|x| {
                tracing::info!(
                    "Finished parsing mapRegions.yaml, task took {:.2}s",
                    start
                        .elapsed()
                        .as_secs_f64()
                );
                x
            })
            .map_err(EveSdeParserError::ParseTypeIds)?;
    let parsed = parsed
        .into_iter()
        .map(|(region_id, wrapper)| {
            Region {
                region_id:         region_id,
                name:              wrapper
                    .name
                    .get("en")
                    .cloned()
                    .unwrap_or_default(),
                constellation_ids: wrapper.constellation_ids,
                position:          wrapper.position,
            }
        })
        .collect::<Vec<_>>();
    Ok(parsed)
}

/// Represents a single entry in the yaml for a type
#[derive(Clone, Debug, Deserialize)]
pub struct RegionWrapper {
    #[serde(rename = "constellationIDs")]
    pub constellation_ids: Vec<ConstellationId>,
    /// Name of the region
    pub name:              HashMap<String, String>,
    pub position:          Position,
}

#[derive(Clone, Debug)]
pub struct Region {
    pub region_id:         RegionId,
    pub name:              String,
    pub constellation_ids: Vec<ConstellationId>,
    pub position:          Position,
}
