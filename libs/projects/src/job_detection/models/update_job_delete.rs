use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateJobDetectionDelete {
    pub delete_from_source: bool,
    pub ignore:             bool,
}
