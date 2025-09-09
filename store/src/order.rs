mod create;
mod delete;
mod error;
mod fetch;
mod list;

use axum::middleware;
use chrono::{DateTime, Utc};
use serde::Serialize;
use starfoundry_lib_eve_gateway::{assert_login, CharacterInfo};
use starfoundry_lib_types::{CharacterId, TypeId};
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use utoipa::ToSchema;

use crate::AppState;
use crate::config::OrderUuid;

pub fn routes() -> OpenApiRouter<AppState> {
    let fetch = OpenApiRouter::new()
        .routes(routes!(fetch::api))
        .route_layer(middleware::from_fn(assert_login));

    let list = OpenApiRouter::new()
        .routes(routes!(list::api))
        .route_layer(middleware::from_fn(assert_login));

    let create = OpenApiRouter::new()
        .routes(routes!(create::api))
        .route_layer(middleware::from_fn(assert_login));

    //let update = OpenApiRouter::new()
    //    .routes(routes!(update::api))
    //    .route_layer(middleware::from_fn(assert_login));

    let delete = OpenApiRouter::new()
        .routes(routes!(delete::api))
        .route_layer(middleware::from_fn(assert_login));

    OpenApiRouter::new()
        .merge(fetch)
        .merge(list)
        .merge(create)
        .merge(delete)
        //.merge(update)
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct Order {
    pub id:                 OrderUuid,
    pub character_id:       CharacterId,
    pub quantity:           i32,
    pub status:             String,
    pub delivery_location:  String,
    pub comment:            Option<String>,
    pub ordered_at:         DateTime<Utc>,

    pub products:           Vec<OrderProduct>,
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct OrderProduct {
    pub name:           String,
    pub price:          i64,
    pub image_type:     String,
    pub image_type_id:  TypeId,
    pub content:        serde_json::Value,
    pub is_additional:  bool,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct OrderResponse {
    pub id:                 OrderUuid,
    pub character:          CharacterInfo,
    pub quantity:           i32,
    pub status:             String,
    pub delivery_location:  String,
    pub comment:            Option<String>,
    pub ordered_at:         DateTime<Utc>,

    pub products:           Vec<OrderProduct>,
}
