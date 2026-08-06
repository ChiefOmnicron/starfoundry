use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{Result, RouteUuid, StructureUuid};
use crate::route::RouteType;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CreateRoute {
    pub name:               String,
    pub typ:                RouteType,

    pub start_structure:    StructureUuid,
    pub end_structure:      StructureUuid,

    pub jump_route:         Option<CreateJumpRoute>,
    pub hauling_route:      Option<CreateHaulingRoute>,
    pub hauling_service:    Option<CreateHaulingService>,
}

impl CreateRoute {
    pub fn validate(&self) -> Result<bool> {
        let one_of = self.jump_route.is_some()
            || self.hauling_route.is_some()
            || self.hauling_service.is_some();

        Ok(one_of)
    }
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CreateJumpRoute {
    pub fuel_usage: i32,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CreateHaulingRoute {
    pub fuel_usage:     i32,
    pub max_cargo_m3:   i32,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CreateHaulingService {
    pub contract_to:        String,
    pub max_cargo_m3:       i32,
    pub price_per_m3:       i32,
    pub collateral_percent: i32,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(
    example = json!({
        "id": "fd324c9f-ecda-49c8-948e-18f4b4b23bff"
    })
)]
pub struct CreateRouteResponse {
    pub id: RouteUuid,
}

impl Default for CreateRouteResponse {
    fn default() -> Self {
        Self {
            id: Uuid::default().into(),
        }
    }
}
