use serde::Deserialize;
use starfoundry_lib_types::{GroupId, CategoryId};
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::time::Instant;

use crate::{FOLDER_INPUT, Error};

pub fn parse(
    directory: &str,
) -> Result<HashMap<GroupId, GroupIdEntry>, Error> {
    tracing::info!("Parsing group.yaml");
    let start = Instant::now();

    let path = format!(
        "{}/{}/groups.yaml",
        directory,
        FOLDER_INPUT,
    );

    if !Path::new(&path).exists() {
        return Err(Error::FileDoesNotExist(path));
    }

    let file = File::open(&path)
        .map_err(|x| Error::CannotOpenGroupIdsFile(x, path))?;

    serde_yaml::from_reader(file)
        .map(|x| {
            tracing::info!(
                "Finished parsing group.yaml, task took {:.2}s",
                start.elapsed().as_secs_f64()
            );
            x
        })
        .map_err(Error::ParseGroupIds)
}

/// Represents a single group entry
#[derive(Clone, Debug, Deserialize)]
pub struct GroupIdEntry {
    /// ID of the group this type belongs to
    #[serde(rename = "categoryID")]
    pub category_id:     CategoryId,
    /// Name of the item in different languages
    #[serde(rename = "name")]
    name:                HashMap<String, String>,
}

impl GroupIdEntry {
    /// Gets the english name for a type.
    ///
    /// # Returns
    ///
    /// If the english translation exists, it is returned, if not [None] is
    /// returned.
    ///
    pub fn name(&self) -> Option<String> {
        self.name.get("en").cloned()
    }
}
