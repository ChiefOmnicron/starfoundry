pub mod category_list;
pub mod create;
pub mod error;
pub mod fetch;
pub mod list;
pub mod update;
pub mod util;

pub use self::error::*;

use axum::middleware;
use serde::Serialize;
use starfoundry_lib_eve_gateway::{assert_admin, assert_login};
use starfoundry_lib_types::TypeId;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use utoipa::ToSchema;

use crate::AppState;
use crate::config::ProductUuid;
use crate::product::create::AdditionalOption;

pub fn routes(
    state: AppState,
) -> OpenApiRouter<AppState> {
    let fetch = OpenApiRouter::new()
        .routes(routes!(fetch::api))
        .route_layer(middleware::from_fn(assert_login));

    let list = OpenApiRouter::new()
        .routes(routes!(list::api))
        .route_layer(middleware::from_fn(assert_login));

    let create = OpenApiRouter::new()
        .routes(routes!(create::api))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_admin))
        .route_layer(middleware::from_fn(assert_login));

    let update = OpenApiRouter::new()
        .routes(routes!(update::api))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_admin))
        .route_layer(middleware::from_fn(assert_login));

    let category_list = OpenApiRouter::new()
        .routes(routes!(category_list::api))
        .route_layer(middleware::from_fn(assert_login));

    OpenApiRouter::new()
        .merge(fetch)
        .merge(list)
        .merge(create)
        .merge(update)
        .merge(category_list)
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct Product {
    /// UUID of the offer
    pub uuid:                   ProductUuid,
    /// Name that should be shown
    pub name:                   String,
    /// Price of the offer
    pub price:                  u64,
    /// Category the offer falls under
    pub category:               String,
    /// Optional description of the product
    pub description:            Option<String>,
    /// TypeId of the structure where the offer can be delivered to
    //pub allowed_stations:       Vec<TypeId>,
    /// List of Characters/Corporation/Alliance that can access the offer
    /// If the list is empty everybody can access the offer
    //pub whitelist:              Vec<i64>,
    /// Tags that can be used for filtering
    pub tags:                   Vec<String>,
    /// Type of the icon that is inserted into the image server
    /// More info: https://developers.eveonline.com/docs/services/image-server/
    pub image_type:             String,
    /// Image that should be shown in the interface
    pub image_type_id:          TypeId,
    /// Content of the product
    pub content:                serde_json::Value,
    /// Additional options for the product
    pub additional_options:     Vec<AdditionalOption>,
}
