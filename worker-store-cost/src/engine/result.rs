use starfoundry_lib_types::TypeId;
use std::collections::HashMap;

use crate::engine::{DependencyTreeEntry, StockMinimal};

pub struct EngineResult {
    pub tree:   HashMap<TypeId, DependencyTreeEntry>,
    pub stocks: Vec<StockMinimal>,
}

impl EngineResult {
    pub fn total_cost(
        &self,
    ) -> f32 {
        self
            .tree
            .iter()
            .map(|(_, x)| x.build_cost.total_job_cost)
            .sum()
    }
}
