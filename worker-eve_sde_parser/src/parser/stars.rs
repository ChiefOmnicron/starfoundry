use serde::{Deserialize, Serialize};
use starfoundry_lib_types::StarId;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::time::Instant;

use crate::{FOLDER_INPUT, Error};

pub fn parse(
    directory: &str,
) -> Result<Vec<Star>, Error> {
    tracing::info!("Parsing mapStars.yaml");
    let start = Instant::now();

    let path = format!(
        "{}/{}/mapStars.yaml",
        directory,
        FOLDER_INPUT,
    );

    if !Path::new(&path).exists() {
        return Err(Error::FileDoesNotExist(path));
    }

    let file = File::open(&path)
        .map_err(|x| Error::CannotOpenTypeIdsFile(x, path))?;

    let parsed: HashMap<StarId, StarWrapper> = serde_yaml::from_reader(file)
        .map(|x| {
            tracing::info!(
                "Finished parsing mapStars.yaml, task took {:.2}s",
                start.elapsed().as_secs_f64()
            );
            x
        })
        .map_err(Error::ParseTypeIds)?;
    let parsed = parsed
        .into_iter()
        .map(|(star_id, wrapper)| Star {
            star_id:     star_id,
            radius:      wrapper.radius,
            temperature: wrapper.statistics.temperature,
        })
        .collect::<Vec<_>>();
    Ok(parsed)
}

/// Represents a single entry in the yaml for a type
#[derive(Clone, Debug, Deserialize)]
pub struct StarWrapper {
    radius:     u32,
    statistics: StarStatisticsWrapper,
}

#[derive(Clone, Debug, Deserialize)]
pub struct StarStatisticsWrapper {
    temperature: f32,
}

#[derive(Clone, Debug, Serialize)]
pub struct Star {
    pub star_id:     StarId,
    pub radius:      u32,
    pub temperature: f32,
}
