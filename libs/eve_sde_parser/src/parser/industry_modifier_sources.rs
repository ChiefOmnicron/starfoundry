use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::time::Instant;

use crate::{FOLDER_INPUT, Error};

pub fn parse(
    directory: &str,
) -> Result<HashMap<usize, ModifyResource>, Error> {
    tracing::info!("Parsing industrymodifiersources.json");
    let start = Instant::now();

    let path = format!(
        "{}/{}/industrymodifiersources.json",
        directory,
        FOLDER_INPUT,
    );

    if !Path::new(&path).exists() {
        return Err(Error::FileDoesNotExist(path));
    }

    let file = File::open(&path)
        .map_err(|x| Error::CannotOpenIndustryModifierSources(x, path))?;

    let data: HashMap<usize, ModifyResource> = serde_json::from_reader(file)
        .map(|x| {
            tracing::info!(
                "Finished parsing industrymodifiersources.json, task took {:.2}s",
                start.elapsed().as_secs_f64()
            );
            x
        })
        .map_err(Error::ParseIndustryModifierSources)?;
    let data = data
        .into_iter()
        .filter(|(_, x)| x.manufacturing.is_some() || x.reaction.is_some())
        .collect::<HashMap<_, _>>();
    Ok(data)
}

#[derive(Clone, Debug, Deserialize)]
pub struct ModifyResource {
    pub manufacturing: Option<Modifier>,
    pub reaction:      Option<Modifier>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Modifier {
    //cost: Option<Vec<ModifierInfo>>,
    pub material: Option<Vec<ModifierInfo>>,
    pub time:     Option<Vec<ModifierInfo>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ModifierInfo {
    #[serde(rename = "dogmaAttributeID")]
    pub attribute: usize,
    #[serde(rename = "filterID")]
    pub filter_id: Option<usize>,
}
