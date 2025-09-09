use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::time::Instant;

use crate::{FOLDER_INPUT, Error};
use starfoundry_lib_types::TypeId;

pub fn parse(
    directory: &str,
) -> Result<HashMap<TypeId, i32>, Error> {
    tracing::info!("Parsing repackagedvolumes.json");
    let start = Instant::now();

    let path = format!(
        "{}/{}/repackagedvolumes.json",
        directory,
        FOLDER_INPUT,
    );

    if !Path::new(&path).exists() {
        return Err(Error::FileDoesNotExist(path));
    }

    let file = File::open(&path)
        .map_err(|x| Error::CannotOpenRepackagedVolumes(x, path))?;

    let data: HashMap<TypeId, f64> = serde_json::from_reader(file)
        .map(|x| {
            tracing::info!(
                "Finished parsing repackagedvolumes.json, task took {:.2}s",
                start.elapsed().as_secs_f64()
            );
            x
        })
        .map_err(Error::ParseRepackagedVolumes)?;
    let data = data
        .into_iter()
        .map(|(type_id, repackaged)| (type_id, repackaged as i32))
        .collect::<HashMap<_, _>>();
    Ok(data)
}
