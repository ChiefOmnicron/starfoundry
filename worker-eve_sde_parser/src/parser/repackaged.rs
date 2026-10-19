use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::time::Instant;

use starfoundry_lib_types::TypeId;

use crate::{EveSdeParserError, FOLDER_INPUT};

pub fn parse(
    directory: &str
) -> Result<HashMap<TypeId, i32>, EveSdeParserError> {
    tracing::info!("Parsing repackagedvolumes.json");
    let start = Instant::now();

    let path =
        format!("{}/{}/repackagedvolumes.json", directory, FOLDER_INPUT,);

    if !Path::new(&path).exists() {
        return Err(EveSdeParserError::FileDoesNotExist(path));
    }

    let file = File::open(&path)
        .map_err(|x| EveSdeParserError::CannotOpenRepackagedVolumes(x, path))?;

    let data: HashMap<TypeId, f64> = serde_json::from_reader(file)
        .map(|x| {
            tracing::info!(
                "Finished parsing repackagedvolumes.json, task took {:.2}s",
                start
                    .elapsed()
                    .as_secs_f64()
            );
            x
        })
        .map_err(EveSdeParserError::ParseRepackagedVolumes)?;
    let data = data
        .into_iter()
        .map(|(type_id, repackaged)| (type_id, repackaged as i32))
        .collect::<HashMap<_, _>>();
    Ok(data)
}
