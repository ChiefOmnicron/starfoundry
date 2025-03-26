use serde::Deserialize;
use starfoundry_libs_types::TypeId;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::time::Instant;

use crate::{FOLDER_INPUT, Error};

pub fn parse(
    directory: &str,
) -> Result<HashMap<TypeId, Vec<TypeMaterial>>, Error> {
    tracing::info!("Parsing typeMaterials.yaml");
    let start = Instant::now();

    let path = format!(
        "{}/{}/fsd/typeMaterials.yaml",
        directory,
        FOLDER_INPUT,
    );

    if !Path::new(&path).exists() {
        return Err(Error::FileDoesNotExist(path));
    }

    let file = File::open(&path)
        .map_err(|x| Error::CannotOpenTypeIdsFile((x, path)))?;

    let parsed: HashMap<TypeId, ParseWrapper> = serde_yaml::from_reader(file)
        .map(|x| {
            tracing::info!(
                "Finished parsing typeMaterials.yaml, task took {:.2}s",
                start.elapsed().as_secs_f64()
            );
            x
        })
        .map_err(Error::ParseTypeIds)?;
    let parsed = parsed
        .into_iter()
        .map(|(type_id, wrapper)| (type_id, wrapper.materials))
        .collect::<HashMap<_, _>>();
    Ok(parsed)
}

/// Represents a single entry in the yaml for a type
#[derive(Clone, Debug, Deserialize)]
pub struct TypeMaterial {
    /// TypeId of the material
    #[serde(rename = "materialTypeID")]
    pub material_type_id: TypeId,
    /// Quantity of the material
    pub quantity:         i32,
}

/// Wrapper only needed for parsing
#[derive(Deserialize)]
struct ParseWrapper {
    materials: Vec<TypeMaterial>,
}
