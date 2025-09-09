use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::time::Instant;

use crate::{FOLDER_INPUT, Error};

pub fn parse(
    directory: &str,
) -> Result<HashMap<usize, DogmaEffect>, Error> {
    tracing::info!("Parsing dogmaEffects.yaml");
    let start = Instant::now();

    let path = format!(
        "{}/{}/dogmaEffects.yaml",
        directory,
        FOLDER_INPUT,
    );

    if !Path::new(&path).exists() {
        return Err(Error::FileDoesNotExist(path));
    }

    let file = File::open(&path)
        .map_err(|x| Error::CannotOpenDogmaEffectsFile(x, path))?;

    let data: HashMap<usize, DogmaEffect> = serde_yaml::from_reader(file)
        .map(|x| {
            tracing::info!(
                "Finished parsing dogmaEffects.yaml, task took {:.2}s",
                start.elapsed().as_secs_f64()
            );
            x
        })
        .map_err(Error::ParseDogmaEffects)?;
    Ok(data)
}

#[derive(Debug, Deserialize)]
pub struct DogmaEffect {
    #[serde(rename = "modifierInfo")]
    pub modifier_info: Option<Vec<DogmaEffectModifier>>,
}

#[derive(Debug, Deserialize)]
pub struct DogmaEffectModifier {
    #[serde(rename = "modifiedAttributeID")]
    pub modified_attribute_id: Option<usize>,
    #[serde(rename = "modifyingAttributeID")]
    pub modifying_attribute_id: Option<usize>,
}

impl DogmaEffectModifier {
    pub fn is_some(&self) -> bool {
        self.modified_attribute_id.is_some() && self.modifying_attribute_id.is_some()
    }
}
