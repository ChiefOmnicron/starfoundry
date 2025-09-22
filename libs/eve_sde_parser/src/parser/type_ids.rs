use serde::{Deserialize, Deserializer};
use starfoundry_libs_types::{TypeId, GroupId};
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::time::Instant;

use crate::{FOLDER_INPUT, Error};

pub fn parse(
    directory: &str,
) -> Result<HashMap<TypeId, TypeIdEntry>, Error> {
    tracing::info!("Parsing type.yaml");
    let start = Instant::now();

    let path = format!(
        "{}/{}/types.yaml",
        directory,
        FOLDER_INPUT,
    );

    if !Path::new(&path).exists() {
        return Err(Error::FileDoesNotExist(path));
    }

    let file = File::open(&path)
        .map_err(|x| Error::CannotOpenTypeIdsFile(x, path))?;

    serde_yaml::from_reader(file)
        .map(|x| {
            tracing::info!(
                "Finished parsing type.yaml, task took {:.2}s",
                start.elapsed().as_secs_f64()
            );
            x
        })
        .map_err(Error::ParseTypeIds)
}

/// Represents a single entry in the yaml for a type
#[derive(Clone, Debug, Deserialize)]
pub struct TypeIdEntry {
    /// ID of the group this type belongs to
    #[serde(rename = "groupID")]
    pub group_id:        GroupId,
    /// Volume of the type
    #[serde(rename = "published")]
    pub published:       bool,
    /// ID of the group this type belongs to
    #[serde(rename = "metaGroupID")]
    #[serde(deserialize_with = "deserialize_meta_group_id")]
    #[serde(default = "default_meta_group_id")]
    pub meta_group_id:   Option<GroupId>,
    /// ID of the market group this type belongs to
    #[serde(rename = "marketGroupID")]
    pub market_group_id: Option<GroupId>,
    /// Volume of the type
    #[serde(rename = "volume")]
    pub volume:          Option<f32>,
    /// Name of the item in different languages
    #[serde(rename = "name")]
    name:                HashMap<String, String>,
}

impl TypeIdEntry {
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

// for some reason some of the "meta_group_id" fields have an float instead of a
// integer, so we make sure to convert it
fn deserialize_meta_group_id<'de, D>(deserializer: D) -> Result<Option<GroupId>, D::Error>
where
    D: Deserializer<'de>,
{
    let field: Option<f32> = Deserialize::deserialize(deserializer)?;
    Ok(field.map(|x| GroupId(x as i32)))
}

fn default_meta_group_id() -> Option<GroupId> {
    None
}
