use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use sqlx::PgPool;
use starfoundry_lib_types::TypeId;

use crate::api_docs::{InternalServerError, NotFound};
use crate::item::{Item, ItemError};
use crate::state::AppState;

use crate::item::error::Result;

/// Fetch an item
/// 
/// - Alternative route: `/latest/items/bulk`
/// - Alternative route: `/v1/items/bulk`
/// 
/// ---
/// 
/// Resolves all information about an item
/// 
#[utoipa::path(
    post,
    path = "/bulk",
    tag = "Items",
    request_body = Vec<TypeId>,
    params(
        TypeId,
    ),
    responses(
        (
            body = Vec<Item>,
            description = "Information about an item",
            status = OK,
        ),
        NotFound,
        InternalServerError,
    ),
)]
pub async fn api(
    State(state):   State<AppState>,
    Json(type_ids): Json<Vec<TypeId>>,
) -> Result<impl IntoResponse> {
    let entry = fetch_item_bulk(
        &state.postgres,
        type_ids,
    ).await?;

    Ok(
        (
            StatusCode::OK,
            Json(entry)
        )
        .into_response()
    )
}

/// Fetches the character information for the given ids from the database.
/// If the character does not exist yet, it will be fetched using the EVE-API.
/// 
pub async fn fetch_item_bulk(
    pool:     &PgPool,
    type_ids: Vec<TypeId>,
) -> Result<Vec<Item>> {
    if type_ids.is_empty() {
        return Ok(Vec::new());
    }

    let type_ids = sqlx::query!("
            SELECT
                type_id,
                category_id,
                group_id,
                volume,
                name,
                meta_group_id,
                repackaged
            FROM item
            WHERE type_id = ANY($1)
            ORDER BY name
        ",
            &type_ids.clone().into_iter().map(|x| *x).collect::<Vec<_>>(),
        )
        .fetch_all(pool)
        .await
        .map_err(ItemError::FetchItemBulk)?
        .into_iter()
        .map(|x| Item {
            category_id:   x.category_id.into(),
            group_id:      x.group_id.into(),
            name:          x.name,
            type_id:       x.type_id.into(),
            volume:        x.volume,

            meta_group_id: x.meta_group_id.map(Into::into),
            repackaged:    x.repackaged,
        })
        .collect::<Vec<_>>();

    Ok(
        type_ids
    )
}
