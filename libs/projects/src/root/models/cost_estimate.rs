use serde::Serialize;
use starfoundry_lib_types::TypeId;
use utoipa::ToSchema;

#[derive(Clone, Debug, Serialize, ToSchema)]

pub struct CostEstimateResponse {
    pub market_cost_total:        f32,
    pub manufacturing_cost_total: f32,
    pub excess_cost_total:        f32,

    pub excess_entries:           Vec<ExcessCostEstimateEntry>,
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct ExcessCostEstimateEntry {
    pub type_id:  TypeId,
    pub quantity: i32,
    pub cost:     f32,
}
