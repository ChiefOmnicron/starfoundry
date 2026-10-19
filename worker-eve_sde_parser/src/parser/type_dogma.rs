use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::time::Instant;

use serde::Deserialize;

use crate::{EveSdeParserError, FOLDER_INPUT};

pub fn parse(
    directory: &str
) -> Result<HashMap<usize, TypeDogma>, EveSdeParserError> {
    tracing::info!("Parsing typeDogma.yaml");
    let start = Instant::now();

    let path = format!("{}/{}/typeDogma.yaml", directory, FOLDER_INPUT,);

    if !Path::new(&path).exists() {
        return Err(EveSdeParserError::FileDoesNotExist(path));
    }

    let file = File::open(&path)
        .map_err(|x| EveSdeParserError::CannotOpenTypeDogmaFile(x, path))?;

    let data: HashMap<usize, TypeDogma> = serde_yaml::from_reader(file)
        .map(|x| {
            tracing::info!(
                "Finished parsing typeDogma.yaml, task took {:.2}s",
                start
                    .elapsed()
                    .as_secs_f64()
            );
            x
        })
        .map_err(EveSdeParserError::ParseTypeDogma)?;
    Ok(data)
}

#[derive(Clone, Debug, Deserialize)]
pub struct TypeDogma {
    #[serde(rename = "dogmaAttributes")]
    pub attributes: Vec<TypeDogmaAttribute>,
    #[serde(rename = "dogmaEffects")]
    #[serde(default)]
    pub effects:    Vec<TypeDogmaEffect>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TypeDogmaAttribute {
    #[serde(rename = "attributeID")]
    pub attribute_id: usize,
    #[serde(rename = "value")]
    pub value:        f32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TypeDogmaEffect {
    #[serde(rename = "effectID")]
    pub effect_id: usize,
}
