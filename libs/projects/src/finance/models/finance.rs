use serde::Serialize;
use utoipa::ToSchema;

/// different finance information per project
#[derive(Debug, Serialize, ToSchema)]
pub struct Finance {
    pub excess:     f64,
    pub jobs:       f64,
    pub market:     f64,
    pub misc:       f64,
    pub stock:      f64,
    pub sell_price: f64,
}
