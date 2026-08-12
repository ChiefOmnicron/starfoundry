use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::time::Instant;

use crate::{FOLDER_INPUT, Error};

pub fn parse(
    directory: &str,
) -> Result<HashMap<usize, Filters>, Error> {
    tracing::info!("Parsing industryTargetFilters.yaml");
    let start = Instant::now();

    let path = format!(
        "{}/{}/industryTargetFilters.yaml",
        directory,
        FOLDER_INPUT,
    );

    if !Path::new(&path).exists() {
        return Err(Error::FileDoesNotExist(path));
    }

    let file = File::open(&path)
        .map_err(|x| Error::CannotOpenIndustryTargetFilters(x, path))?;

    let data: HashMap<usize, Filters> = serde_yaml::from_reader(file)
        .map(|x| {
            tracing::info!(
                "Finished parsing industryTargetFilters.yaml, task took {:.2}s",
                start.elapsed().as_secs_f64()
            );
            x
        })
        .map_err(Error::ParseIndustryTargetFilters)?;
    Ok(data)
}

#[derive(Debug, Deserialize)]
pub struct Filters {
    #[serde(rename = "categoryIDs")]
    #[serde(default)]
    pub category_ids:   Vec<usize>,
    #[serde(rename = "groupIDs")]
    #[serde(default)]
    pub group_ids:      Vec<usize>,
}
