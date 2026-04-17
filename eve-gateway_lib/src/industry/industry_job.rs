use serde::{Deserialize, Deserializer, Serialize};
use utoipa::ToSchema;
use starfoundry_lib_types::{CharacterId, CorporationId, ItemId, JobId, LocationId, TypeId};


#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct IndustryJob {
    /// Activity of the job
    #[serde(deserialize_with = "IndustryActivity::from")]
    #[serde(rename = "activity_id")]
    pub activity:               IndustryActivity,
    /// Asset ID of the blueprint
    pub blueprint_id:           ItemId,
    /// ID of the blueprint
    pub blueprint_type_id:      TypeId,
    /// ID of the blueprint
    pub product_type_id:        TypeId,
    /// Station where the blueprint is located, can also be containers
    pub blueprint_location_id:  LocationId,
    /// When the job is done
    pub end_date:               String,
    /// Cost of the job
    pub cost:                   Option<f32>,
    /// Number of runs
    pub runs:                   i32,
    /// Number of runs blueprint is licensed for
    /// For copying for example this will be the amount of copies you are creating
    pub licensed_runs:          i32,
    /// Unique id for the job
    pub job_id:                 JobId,
    /// ID of the facility the job was started in
    pub facility_id:            i64,
    /// CharacterId of the character that strted the job
    pub installer_id:           CharacterId,
    /// Status of the manufacturing entry
    pub status:                 String,
    /// ID of the location for the industry facility
    pub location_id:            LocationId,
    /// Location ID of the location to which the output of the job will be delivered. Normally a station ID, but can also be a corporation facility
    pub output_location_id:     LocationId,
    /// [CorporationId] of the corporation that owns this job
    #[serde(default)]
    pub corporation_id:         Option<CorporationId>,
}

/// List of all industry activities
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum IndustryActivity {
    /// Manufacturing of things
    Manufacturing,
    /// Researching time efficiency
    MaterialEfficiencyResearch,
    /// Researching material efficiency
    TimeEfficiencyResearch,
    /// Making blueprint copies
    Copying,
    /// The process of creating a more advanced item based on an existing item
    Invention,
    /// The process of combining raw and intermediate materials to create advanced components
    Reactions,
    /// No matches were found
    Unknown,
}

impl IndustryActivity {
    /// deserializes industry activity ids into their actual activitiy name
    fn from<'de, D>(d: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(match u8::deserialize(d)? {
            1 => Self::Manufacturing,
            3 => Self::MaterialEfficiencyResearch,
            4 => Self::TimeEfficiencyResearch,
            5 => Self::Copying,
            8 => Self::Invention,
            9 => Self::Reactions,
            _ => Self::Unknown,
        })
    }
}
