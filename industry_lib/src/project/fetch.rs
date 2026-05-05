use serde::{Deserialize, Serialize};
use starfoundry_lib_eve_gateway::Item;
use utoipa::ToSchema;
use uuid::Uuid;
use starfoundry_lib_market::MarketBulkResponse;

use crate::{ProjectUuid, SolutionUuid};
use crate::project::{ProjectExcess, ProjectStatus};
use crate::project_group::ProjectGroup;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(
    example = json!({
        "id": "b034c3a9-2f4d-487d-95bb-c66fc20148b3",
        "name": "My cool project",
        "status": "IN_PROGRESS",
        "orderer": "Me Myself and I",
        "sell_price": 1337
    })
)]
pub struct Project {
    pub id:             ProjectUuid,
    pub name:           String,
    pub status:         ProjectStatus,
    pub orderer:        String,
    pub project_group:  ProjectGroup,
    pub products:       Vec<ProjectProduct>,
    pub stock:          Vec<ProjectStock>,
    pub excess:         Vec<ProjectExcess>,

    pub note:           Option<String>,
    pub sell_price:     Option<f64>,
    #[serde(skip)]
    pub solution_id:    Option<SolutionUuid>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct ProjectProduct {
    pub item:                   Item,
    pub quantity:               i32,
    pub material_efficiency:    i32,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ProjectCost {
    pub sell_price:     f64,

    pub job_cost:       f64,
    pub market_cost:    f64,
    pub misc_cost:      f64,
    pub excess_cost:    f64,
    pub stock_cost:     f64,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ProjectTimeLeft {
    pub state:      String,
    pub date_ms:    i64,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct ProjectMarket {
    pub id:       Uuid,
    pub item:     Item,
    pub quantity: i32,

    pub cost:     Option<f64>,
    pub source:   Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct ProjectMarketBuy {
    pub id:       Uuid,
    pub item:     Item,
    pub quantity: i32,

    pub cost:     Option<f64>,
    pub source:   Option<String>,

    pub entries:  Vec<MarketBulkResponse>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct ProjectMisc {
    pub id:          Uuid,
    pub item:        String,
    pub cost:        f64,

    pub description: Option<String>,
    pub quantity:    Option<i32>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct ProjectStock {
    pub item:       Item,
    pub quantity:   i32,
    pub cost:       Option<f64>,
}
