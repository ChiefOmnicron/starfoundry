mod error;
pub mod fetch;
pub mod fetch_bulk;

pub use self::error::*;

use serde::{Deserialize, Serialize};
use starfoundry_lib_types::{CategoryId, GroupId, TypeId};
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use utoipa::ToSchema;

use crate::state::AppState;

/// Exposes all routes that are under `/items`
pub fn routes() -> OpenApiRouter<AppState> {
    let fetch = OpenApiRouter::new()
        .routes(routes!(fetch::api));

    OpenApiRouter::new()
        .merge(fetch)
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
#[schema(
    example = json!({
        "base_price": null,
        "category_id": 6,
        "group_id": 30,
        "meta_group_id": null,
        "name": "Ragnarok",
        "repackaged": 10000000,
        "type_id": 23773,
        "volume": 100000000
    })
)]
pub struct Item {
    pub type_id:        TypeId,
    pub category_id:    CategoryId,
    pub group_id:       GroupId,
    pub volume:         f32,
    pub name:           String,

    pub meta_group_id:  Option<GroupId>,
    pub repackaged:    Option<i32>,
}
