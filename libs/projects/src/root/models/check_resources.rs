use serde::Deserialize;
use utoipa::ToSchema;

use crate::{ProjectJobUuid, StockMinimal};

#[derive(Debug, Deserialize, ToSchema)]
pub struct CheckResources {
    /// jobs that should be started
    pub job_ids:   Vec<ProjectJobUuid>,
    /// available materials
    pub resources: Vec<StockMinimal>,
}
