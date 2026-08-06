use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum RouteType {
    JumpRoute,
    HaulingRoute,
    HaulingService,
}

impl RouteType {
    pub fn as_str(&self) -> &str {
        match *self {
            Self::JumpRoute         => "JUMP_ROUTE",
            Self::HaulingRoute      => "HAULING_ROUTE",
            Self::HaulingService    => "HAULING_SERVICE",
        }
    }
}

impl TryFrom<String> for RouteType {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_ref() {
            "JUMP_ROUTE"        => Ok(RouteType::JumpRoute),
            "HAULING_ROUTE"     => Ok(RouteType::HaulingRoute),
            "HAULING_SERVICE"   => Ok(RouteType::HaulingService),
            _                   => Err("Invalid route type".into())
        }
    }
}
