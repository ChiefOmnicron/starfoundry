use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::filter::FilterType;

#[derive(Debug, Deserialize, Serialize, ToSchema, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct FilterQuery {
    pub typ: FilterType,
}
