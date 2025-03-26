use serde::Serialize;
use starfoundry_libs_types::TypeId;
use std::collections::HashMap;

#[derive(Clone, Debug, Default, Serialize)]
pub struct DependencyBuildCost {
    pub total_job_gross:         f32,
    pub material_cost_total:     f32,
    pub facility:                f32,
    pub scc:                     f32,
    pub total_job_cost:          f32,
    pub material_adjusted_price: HashMap<TypeId, f32>,
}
