mod error;
mod job_planner;

use serde::{Deserialize, Serialize};
use starfoundry_eve_gateway::asset::Item;
use starfoundry_gateway::ApiClient;
use starfoundry_lib_types::{StructureId, TypeId};
use utoipa::ToSchema;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

pub use self::job_planner::*;
use crate::industry_hub::IndustryHub;
use crate::structure::Structure;
use crate::{AppState, ProjectGroupUuid, ProjectJobUuid, Result, SolutionUuid};

pub fn routes() -> OpenApiRouter<AppState> {
    let calculation = OpenApiRouter::new().routes(routes!(job_planner::api));

    OpenApiRouter::new().merge(calculation)
}

pub trait IndustryApiClientIndustry: ApiClient {
    #[allow(async_fn_in_trait)]
    async fn calculation(
        &self,
        request: &BuildEngine,
    ) -> Result<Vec<BuildEngineResponse>> {
        self.post("industry/calculation", request)
            .await
            .map_err(Into::into)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct BuildEngine {
    pub project_group_id:        ProjectGroupUuid,
    pub products:                Option<Vec<BuildEngineProduct>>,
    pub products_str:            Option<String>,
    pub additional_products:     Option<Vec<BuildEngineAdditionalProduct>>,
    pub additional_products_str: Option<String>,

    pub stocks:     Option<Vec<StockMinimal>>,
    pub stocks_str: Option<String>,

    pub blacklist:             Option<Vec<TypeId>>,
    pub blueprint_overwrite:   Option<Vec<TmpBlueprintOverwrite>>,
    pub job_splitting:         Option<Vec<TmpJobSplitting>>,
    pub markets:               Option<Vec<StructureId>>,
    pub calculate_market_cost: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct BuildEngineProduct {
    pub type_id:             TypeId,
    pub material_efficiency: u32,
    pub quantity:            u32,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct TmpBlueprintOverwrite {
    pub type_id:             TypeId,
    pub material_efficiency: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct TmpJobSplitting {
    pub type_id: TypeId,
    pub runs:    u32,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct BuildEngineResponse {
    pub solution_id:   SolutionUuid,
    pub industry_hub:  IndustryHub,
    pub material:      Vec<BuildEngineMaterialResponse>,
    pub manufacturing: Vec<BuildEngineManufacturingResponse>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct BuildEngineMaterialResponse {
    pub item:   Item,
    pub needed: f32,
    pub stock:  i32,
    pub price:  Option<f64>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct BuildEngineAdditionalProduct {
    pub type_id:  TypeId,
    pub quantity: u32,
    pub price:    Option<f64>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct BuildEngineManufacturingResponse {
    pub id:        ProjectJobUuid,
    pub item:      Item,
    pub runs:      Vec<u32>,
    pub structure: Option<Structure>,
    pub build_tax: f32,
    pub time:      f32,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, ToSchema)]
#[schema(
    example = json!({
        "quantity": 3315,
        "type_id": 11399,
    })
)]
pub struct StockMinimal {
    pub quantity: i32,
    pub type_id:  TypeId,
}
