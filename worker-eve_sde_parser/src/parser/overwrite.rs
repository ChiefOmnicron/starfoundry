use serde::Deserialize;
use starfoundry_lib_types::TypeId;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::time::Instant;

use crate::parser::blueprints::BlueprintEntry;
use crate::Error;
use crate::parser::type_ids::TypeIdEntry;

pub fn parse(
    directory: &str,
) -> Result<Overwrites, Error> {
    tracing::info!("Parsing blueprints.yaml");
    let start = Instant::now();

    let path = format!(
        "{}/overwrites.yaml",
        directory,
    );

    if !Path::new(&path).exists() {
        return Err(Error::FileDoesNotExist(path));
    }

    let file = File::open(&path)
        .map_err(|x| Error::CannotOpenBlueprintsFile(x, path))?;

    serde_yaml::from_reader(file)
        .map(|x| {
            tracing::info!(
                "Finished parsing overwrites.yaml, task took {:.2}s",
                start.elapsed().as_secs_f64()
            );
            x
        })
        .map_err(Error::ParseBlueprints)
}

#[derive(Clone, Debug, Deserialize)]
pub struct Overwrites {
    pub items:      HashMap<TypeId, TypeIdEntry>,
    pub blueprints: HashMap<TypeId, BlueprintEntry>,
}
