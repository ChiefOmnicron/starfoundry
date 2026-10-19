use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::time::Instant;

use serde::Deserialize;

use crate::{EveSdeParserError, FOLDER_INPUT};

pub fn parse(
    directory: &str
) -> Result<HashMap<usize, Filters>, EveSdeParserError> {
    tracing::info!("Parsing industryTargetFilters.yaml");
    let start = Instant::now();

    let path =
        format!("{}/{}/industryTargetFilters.yaml", directory, FOLDER_INPUT,);

    if !Path::new(&path).exists() {
        return Err(EveSdeParserError::FileDoesNotExist(path));
    }

    let file = File::open(&path).map_err(|x| {
        EveSdeParserError::CannotOpenIndustryTargetFilters(x, path)
    })?;

    let data: HashMap<usize, Filters> = serde_yaml::from_reader(file)
        .map(|x| {
            tracing::info!(
                "Finished parsing industryTargetFilters.yaml, task took {:.2}s",
                start
                    .elapsed()
                    .as_secs_f64()
            );
            x
        })
        .map_err(EveSdeParserError::ParseIndustryTargetFilters)?;
    Ok(data)
}

#[derive(Debug, Deserialize)]
pub struct Filters {
    #[serde(rename = "categoryIDs")]
    #[serde(default)]
    pub category_ids: Vec<usize>,
    #[serde(rename = "groupIDs")]
    #[serde(default)]
    pub group_ids:    Vec<usize>,
}
