use serde::{Deserialize, Serialize};
use starfoundry_lib_types::{SystemId, StructureId, TypeId};

use crate::{Cache, Error, EveApiClient};

impl EveApiClient {
    /// Fetches information about the given location id.
    /// The location id must be larget than 1_000_000_000_000.
    /// 
    /// # Errors
    /// 
    /// - If the EVE API is not available
    /// - If the [EveAuthClient] is not valid
    /// - If the character does not have access to the structure
    /// - If the structure does not exist
    /// - If the [StructureId] is not a valid id
    /// 
    /// # Returns
    /// Information about the structure
    /// 
    pub async fn resolve_structure(
        &self,
        structure_id: StructureId,
    ) -> Result<(StructureId, Structure), Error> {
        let path = format!(
            "latest/universe/structures/{}",
            structure_id
        );

        let response = self
            .fetch_auth(&path, Cache::Ignore)
            .await
            .map(|x| (structure_id, x))
            .map_err(Into::into)?;

        Ok(response)
    }
}

/// Represents a strucutre
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Structure {
    /// Name of the structure
    pub name:      String,
    /// Id of the system the structure is located in
    #[serde(alias = "solar_system_id")]
    pub system_id: SystemId,
    /// [TypeId] of the structure
    pub type_id:   TypeId,
}

/// Represents a System
#[derive(Clone, Debug, Deserialize)]
pub struct System {
    /// Name of the system
    pub name:            String,
    /// Securtiy status of the system
    pub security_status: f32,
    /// Id of the system
    pub system_id:       SystemId,
}

/// Represents the different industry activities in a system
#[derive(Clone, Debug, Deserialize)]
pub struct IndustrySystem {
    /// List of the different activities and their index
    pub cost_indices:    Vec<IndustrySystemIndex>,
    /// If of the system
    pub solar_system_id: u32,
}

impl IndustrySystem {
    /// Gets all activities for a single system
    /// 
    /// # Returns
    /// 
    /// * `0` > manufacturing
    /// * `1` > reaction
    /// * `2` > copying
    /// * `3` > invention
    /// * `4` > researching_material_efficiency
    /// * `5` > researching_time_efficiency
    pub fn index_by_activity(
        &self
    ) -> (f32, f32, f32, f32, f32, f32) {
        let mut manufacturing = 0f32;
        let mut reaction      = 0f32;
        let mut copying       = 0f32;
        let mut invention     = 0f32;
        let mut rme           = 0f32;
        let mut rte           = 0f32;

        for index in self.cost_indices.iter() {
            match index.activity.as_ref() {
                "manufacturing"                   => manufacturing = index.cost_index,
                "reaction"                        => reaction = index.cost_index,
                "copying"                         => copying = index.cost_index,
                "invention"                       => invention = index.cost_index,
                "researching_material_efficiency" => rme = index.cost_index,
                "researching_time_efficiency"     => rte = index.cost_index,
                _ => continue
            }
        }
        (
            manufacturing,
            reaction,
            copying,
            invention,
            rme,
            rte
        )
    }
}

/// Represents a industry index activity
#[derive(Clone, Debug, Deserialize)]
pub struct IndustrySystemIndex {
    /// One of copying, duplicating, invention, manufacturing, none, reaction,
    /// researching_material_efficiency, researching_technology,
    /// researching_time_efficiency, reverse_engineering
    pub activity:   String,
    /// Index of the activity
    pub cost_index: f32,
}
