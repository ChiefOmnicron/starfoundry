use serde::{Deserialize, Serialize};
use starfoundry_lib_eve_gateway::Item;
use starfoundry_lib_types::TypeId;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{ProjectJobUuid, StructureUuid};


/// Either `materials` or `materials_str` is required
/// If `materials_str` is given, they will be resolved to their type_id and quantity
/// 
#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct CheckMaterialsRequest {
    pub job_ids:        Vec<ProjectJobUuid>,
    pub materials:      Option<Vec<Material>>,
    pub materials_str:  Option<String>,
}

#[derive(Clone, Default, Debug, Deserialize, Serialize, ToSchema)]
pub struct CheckMaterialsResponse {
    pub job_cost:   f32,
    pub materials:  Vec<CheckMaterialsResponseMaterial>,
    pub blueprints: Vec<CheckMaterialsResponseBlueprint>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct CheckMaterialsResponseMaterial {
    pub item:     Item,
    pub quantity: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct CheckMaterialsResponseBlueprint {
    // only needed for sorting
    #[serde(skip)]
    pub id:       Uuid,
    pub item:     Item,
    pub runs:     Vec<u32>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct Material {
    pub quantity: i32,
    pub type_id:  TypeId,
}

#[derive(Clone, Debug)]
pub struct JobToStart {
    pub runs:           i32,
    pub type_id:        TypeId,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct SplitJobRequest {
    pub old: SplitJobEntry,
    pub new: Vec<SplitJobEntry>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct SplitJobEntry {
    pub type_id: TypeId,
    pub runs:    u32,
}

#[derive(Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct SplitJobResponse {
    pub excess:     Vec<SplitJobResponseMarketEntry>,
    pub materials:  Vec<SplitJobResponseMarketEntry>,
    pub jobs:       Vec<SplitJobResponseJobEntry>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct SplitJobResponseJobEntry {
    #[serde(skip)]
    pub id:             Uuid,
    pub item:           Item,
    pub runs:           i32,
    pub structure_id:   StructureUuid,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct SplitJobResponseMarketEntry {
    pub item:       Item,
    pub quantity:   i32,
}
