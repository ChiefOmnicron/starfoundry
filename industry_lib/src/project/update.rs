use serde::{Deserialize, Serialize};
use starfoundry_lib_market::{GasDecompressionEfficiency, OreReprocessingEfficiency};
use starfoundry_lib_types::{StructureId, TypeId};
use utoipa::ToSchema;

use crate::project::{ProjectJobStatus, ProjectStatus};
use crate::TagUuid;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct UpdateProject {
    pub orderer:        String,
    pub name:           String,
    pub status:         ProjectStatus,
    pub tags:           Vec<TagUuid>,

    pub sell_price:     Option<f64>,
    pub note:           Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateJob {
    pub cost:   Option<f64>,
    pub job_id: Option<i32>,
    pub runs:   Option<i32>,
    pub status: ProjectJobStatus,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct UpdateMarketBulk {
    pub source:                 String,
    pub entries:                Vec<UpdateMarketBulkEntry>,
    #[serde(default)]
    pub gas_decompression:      Option<GasDecompressionEfficiency>,
    #[serde(default)]
    pub mineral_compression:    Option<OreReprocessingEfficiency>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct UpdateMarketBulkEntry {
    #[serde(default)]
    pub type_id:        Option<TypeId>,
    #[serde(default)]
    pub name:           Option<String>,
    #[serde(default)]
    pub structure_id:   Option<StructureId>,

    pub cost:           f32,
    pub quantity:       i32,
}


#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct UpdateMarketEntry {
    pub quantity:   i32,
    pub cost:       Option<f64>,
    pub source:     Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct UpdateMisc {
    pub item:           String,
    pub cost:           f32,

    pub description:    Option<String>,
    pub quantity:       Option<i32>,
}
