use serde::Deserialize;
use utoipa::ToSchema;

use crate::AppraisalList;

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateExcessPrice {
    pub appraisal: AppraisalList,
}
