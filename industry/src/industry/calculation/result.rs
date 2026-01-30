use serde::Serialize;
use starfoundry_lib_types::TypeId;
use std::{collections::HashMap, fs::File};

use crate::industry::calculation::models::{DependencyTreeEntry, StockMinimal};
use crate::industry::calculation::project_config::ProjectConfig;

#[derive(Debug, Serialize)]
pub struct EngineResult {
    pub tree:   HashMap<TypeId, DependencyTreeEntry>,
    pub config: ProjectConfig,
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

    /// Writes the current tree to disk
    /// 
    #[allow(unused)]
    pub fn write_debug_file(&self) {
        let mut file = File::create("FlatTreeDebug.json").unwrap();
        serde_json::to_writer_pretty(&mut file, &self).unwrap();
    }
}
