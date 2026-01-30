use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::System;

/// Represents the different industry activities in a system
#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
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
    ) -> IndustrySystemActivity {
        let mut manufacturing        = 0f32;
        let mut reaction             = 0f32;
        let mut copying              = 0f32;
        let mut invention            = 0f32;
        let mut researching_material = 0f32;
        let mut researching_time     = 0f32;

        for index in self.cost_indices.iter() {
            match index.activity.as_ref() {
                "manufacturing"        => manufacturing = index.cost_index,
                "reaction"             => reaction = index.cost_index,
                "copying"              => copying = index.cost_index,
                "invention"            => invention = index.cost_index,
                "researching_material" => researching_material = index.cost_index,
                "researching_time"     => researching_time = index.cost_index,
                _ => continue
            }
        }

        IndustrySystemActivity {
            manufacturing,
            reaction,
            copying,
            invention,
            researching_material,
            researching_time,
        }
    }
}

/// Represents a industry index activity
#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct IndustrySystemIndex {
    /// One of copying, duplicating, invention, manufacturing, none, reaction,
    /// researching_material_efficiency, researching_technology,
    /// researching_time_efficiency, reverse_engineering
    pub activity:   String,
    /// Index of the activity
    pub cost_index: f32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IndustrySystemActivity {
    pub manufacturing:        f32,
    pub reaction:             f32,
    pub copying:              f32,
    pub invention:            f32,
    pub researching_material: f32,
    pub researching_time:     f32,
}

/// System index by SystemId
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct SystemIndex {
    pub system:               System,
    pub manufacturing:        f32,
    pub reaction:             f32,
    pub copying:              f32,
    pub invention:            f32,
    pub researching_material: f32,
    pub researching_time:     f32,
}
