use chrono::{DateTime, Utc};
use serde::Serialize;
use starfoundry_lib_eve_gateway::CharacterInfo;
use utoipa::ToSchema;

use crate::config::OrderUuid;
use crate::order::OrderProduct;

#[derive(Debug, Serialize, ToSchema)]
pub struct AdminOrderResponse {
    pub id:                     OrderUuid,
    pub character:              CharacterInfo,
    pub quantity:               i32,
    pub status:                 String,
    pub delivery_location:      String,
    pub comment:                Option<String>,
    pub ordered_at:             DateTime<Utc>,
    pub sf_industry_link:       Option<String>,
    pub expected_delivery_date: Option<DateTime<Utc>>,

    pub products:               Vec<OrderProduct>,
}
