use serde::{Deserialize, Serialize};
use starfoundry_lib_eve_gateway::Item;
use starfoundry_lib_types::TypeId;
use utoipa::ToSchema;

use crate::{ProjectGroupUuid, ProjectJobUuid, SolutionUuid};
use crate::industry_hub::IndustryHub;
use crate::industry::StockMinimal;
use crate::structure::Structure;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct BuildEngine {
    pub project_group_id:       ProjectGroupUuid,
    pub products:               Option<Vec<BuildEngineProduct>>,
    pub products_str:           Option<String>,

    pub stocks:                 Option<Vec<StockMinimal>>,
    pub stocks_str:             Option<String>,

    pub blacklist:              Option<Vec<TypeId>>,
    pub blueprint_overwrite:    Option<Vec<TmpBlueprintOverwrite>>,
    pub job_splitting:          Option<Vec<TmpJobSplitting>>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct BuildEngineProduct {
    pub type_id:                TypeId,
    pub material_efficiency:    u32,
    pub quantity:               u32,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct TmpBlueprintOverwrite {
    pub type_id:                TypeId,
    pub material_efficiency:    u32,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct TmpJobSplitting {
    pub type_id:    TypeId,
    pub runs:       u32,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct BuildEngineResponse {
    pub solution_id:    SolutionUuid,
    pub industry_hub:   IndustryHub,
    pub material:       Vec<BuildEngineMaterialResponse>,
    pub manufacturing:  Vec<BuildEngineManufacturingResponse>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct BuildEngineMaterialResponse {
    pub item:   Item,
    pub needed: f32,
    pub stock:  i32,
    // TODO: add market
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct BuildEngineManufacturingResponse {
    pub id:         ProjectJobUuid,
    pub item:       Item,
    pub runs:       Vec<u32>,
    pub structure:  Option<Structure>,
    pub build_tax:  f32,
    pub time:       f32,
}
