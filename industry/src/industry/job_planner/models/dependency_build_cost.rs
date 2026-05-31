use serde::Serialize;
use starfoundry_lib_types::TypeId;
use std::collections::HashMap;

#[derive(Clone, Debug, Default, Serialize)]
pub struct DependencyBuildCost {
    pub base_item_cost:          f32,

    pub system_cost_percent:     f32,
    pub system_cost:             f32,
    pub total_job_gross:         f32,

    pub facility:                f32,
    pub facility_percent:        f32,
    pub scc:                     f32,
    pub scc_percent:             f32,
    pub total_tax:               f32,

    pub total_job_cost:          f32,
    pub material_adjusted_price: HashMap<TypeId, f32>,
}
