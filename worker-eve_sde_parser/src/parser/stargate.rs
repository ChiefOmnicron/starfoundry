use serde::Deserialize;
use starfoundry_lib_types::{StargateId, SystemId, TypeId};
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::time::Instant;

use crate::parser::systems::Position;
use crate::{FOLDER_INPUT, Error};

pub fn parse(
    directory: &str,
) -> Result<Vec<Stargate>, Error> {
    tracing::info!("Parsing mapStargates.yaml");
    let start = Instant::now();

    let path = format!(
        "{}/{}/mapStargates.yaml",
        directory,
        FOLDER_INPUT,
    );

    if !Path::new(&path).exists() {
        return Err(Error::FileDoesNotExist(path));
    }

    let file = File::open(&path)
        .map_err(|x| Error::CannotOpenTypeIdsFile(x, path))?;

    let parsed: HashMap<StargateId, StargateWrapper> = serde_yaml::from_reader(file)
        .map(|x| {
            tracing::info!(
                "Finished parsing mapStargates.yaml, task took {:.2}s",
                start.elapsed().as_secs_f64()
            );
            x
        })
        .map_err(Error::ParseTypeIds)?;
    let parsed = parsed
        .into_iter()
        .map(|(stargate_id, wrapper)| Stargate {
            stargate_id:    stargate_id,
            destination:    StargateDestination {
                                system_id:      wrapper.destination.system_id,
                                stargate_id:    wrapper.destination.stargate_id,
                            },
            position:       wrapper.position,
            system_id:      wrapper.system_id,
            type_id:        wrapper.type_id,
        })
        .collect::<Vec<_>>();
    Ok(parsed)
}

/// Represents a single entry in the yaml for a type
#[derive(Clone, Debug, Deserialize)]
pub struct StargateWrapper {
    pub destination:    StargateDestinationWrapper,
    pub position:       Position,
    #[serde(rename = "solarSystemID")]
    pub system_id:      SystemId,
    #[serde(rename = "typeID")]
    pub type_id:        TypeId,
}

#[derive(Clone, Debug, Deserialize)]
pub struct StargateDestinationWrapper {
    #[serde(rename = "solarSystemID")]
    pub system_id:      SystemId,
    #[serde(rename = "stargateID")]
    pub stargate_id:    StargateId,
}

#[derive(Clone, Debug)]
pub struct Stargate {
    pub stargate_id:    StargateId,
    pub destination:    StargateDestination,
    pub position:       Position,
    pub system_id:      SystemId,
    pub type_id:        TypeId,
}

#[derive(Clone, Debug)]
pub struct StargateDestination {
    pub system_id:      SystemId,
    pub stargate_id:    StargateId,
}
