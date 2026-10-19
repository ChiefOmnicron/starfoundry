pub mod clone;
pub mod create;
pub mod delete;
pub mod error;
pub mod fetch;
pub mod list;
pub mod permission;
pub mod update;

use axum::middleware;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

pub use self::error::*;
pub use self::fetch::fetch;
use crate::industry_hub::permission::{
    assert_exists, assert_read, assert_write
};
use crate::structure::Structure;
use crate::{AppState, IndustryHubUuid};

pub fn routes(state: AppState) -> OpenApiRouter<AppState> {
    let list = OpenApiRouter::new().routes(routes!(list::api));

    let fetch = OpenApiRouter::new()
        .routes(routes!(fetch::api))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_read))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            assert_exists,
        ));

    let create = OpenApiRouter::new().routes(routes!(create::api));

    let update = OpenApiRouter::new()
        .routes(routes!(update::api))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            assert_write,
        ))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            assert_exists,
        ));

    let delete = OpenApiRouter::new()
        .routes(routes!(delete::api))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            assert_write,
        ))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            assert_exists,
        ));

    let clone = OpenApiRouter::new()
        .routes(routes!(clone::api))
        //.route_layer(middleware::from_fn_with_state(state.clone(), assert_shared))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            assert_exists,
        ));

    OpenApiRouter::new()
        .merge(list)
        .merge(fetch)
        .merge(create)
        .merge(update)
        .merge(clone)
        .merge(delete)
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct IndustryHub {
    pub id:          IndustryHubUuid,
    pub name:        String,
    pub structures:  Vec<Structure>,
    pub shares:      Vec<IndustryHubShare>,
    pub description: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
#[schema(
    examples(
        json!({
            "name": "Rational Chaos Inc.",
            "share_id": 98024275,
            "share_type": "CORPORATION"
        })
    )
)]
pub struct IndustryHubShare {
    pub name:       String,
    /// either a character id, corporation id or alliance id
    pub share_id:   i32,
    pub share_type: IndustryHubShareType,
}

#[derive(
    Clone,
    Debug,
    Copy,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    sqlx::Type,
    Deserialize,
    Serialize,
    ToSchema,
)]
#[sqlx(type_name = "SHARE_TYPE")]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum IndustryHubShareType {
    Character,
    Corporation,
    Alliance,
}

#[cfg(test)]
pub async fn industry_hub_test_routes(
    postgres: sqlx::PgPool,
    request: axum::http::Request<axum::body::Body>,
) -> axum::http::Response<axum::body::Body> {
    use std::sync::Arc;

    use tower::ServiceExt;

    use crate::metrics::Metric;

    let state: AppState = AppState {
        postgres: postgres.clone(),
        metric:   Arc::new(Metric::new()),
    };
    let (app, _) = crate::industry_hub::routes(state.clone()).split_for_parts();
    let app = app.with_state(state.clone());

    app.clone()
        .oneshot(request)
        .await
        .unwrap()
}
