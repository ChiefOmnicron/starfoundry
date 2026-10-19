use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::time::Instant;

use serde::Deserialize;
use starfoundry_lib_types::CategoryId;

use crate::{EveSdeParserError, FOLDER_INPUT};

pub fn parse(
    directory: &str
) -> Result<HashMap<CategoryId, CategoryIdEntry>, EveSdeParserError> {
    tracing::info!("Parsing categories.yaml");
    let start = Instant::now();

    let path = format!("{}/{}/categories.yaml", directory, FOLDER_INPUT,);

    if !Path::new(&path).exists() {
        return Err(EveSdeParserError::FileDoesNotExist(path));
    }

    let file = File::open(&path)
        .map_err(|x| EveSdeParserError::CannotOpenTypeIdsFile(x, path))?;

    serde_yaml::from_reader(file)
        .map(|x| {
            tracing::info!(
                "Finished parsing categories.yaml, task took {:.2}s",
                start
                    .elapsed()
                    .as_secs_f64()
            );
            x
        })
        .map_err(EveSdeParserError::ParseTypeIds)
}

/// Represents a single entry in the yaml for a type
#[derive(Clone, Debug, Deserialize)]
pub struct CategoryIdEntry {
    /// Name of the item in different languages
    #[serde(rename = "name")]
    name: HashMap<String, String>,
}

impl CategoryIdEntry {
    /// Gets the english name for a type.
    ///
    /// # Returns
    ///
    /// If the english translation exists, it is returned, if not [None] is
    /// returned.
    pub fn name(&self) -> Option<String> {
        self.name
            .get("en")
            .cloned()
    }
}
