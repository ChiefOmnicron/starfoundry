use std::collections::HashMap;

use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use starfoundry_lib_types::{OrderId, StructureId, TypeId};
use utoipa::{IntoParams, ToSchema};

use crate::AppState;
use crate::api_docs::{BadRequest, InternalServerError, Unauthorized};
use crate::market::error::Result;

/// Update Virtual
///
/// - Alternative route: `/latest/markets/virtual`
/// - Alternative route: `/v1/markets/virtual`
///
/// ---
///
/// Updates the virtual remaining quantity for market orders
#[utoipa::path(
    post,
    path = "/virtual",
    tag = "Markets",
    request_body = Vec<MarketVirtualRequest>,
    responses(
        (
            description = "No markets have the requested amount available",
            status = NO_CONTENT,
        ),
        BadRequest,
        Unauthorized,
        InternalServerError,
    ),
)]
#[axum::debug_handler]
pub async fn api(
    State(state): State<AppState>,
    Json(request): Json<Vec<MarketVirtualRequest>>,
) -> Result<impl IntoResponse> {
    virtual_market(&state.postgres, request).await?;

    Ok((StatusCode::NO_CONTENT, Json(())).into_response())
}

pub async fn virtual_market(
    pool: &PgPool,
    entries: Vec<MarketVirtualRequest>,
) -> Result<()> {
    let type_ids = entries
        .iter()
        .map(|x| x.type_id)
        .collect::<Vec<_>>();
    let structure_ids = entries
        .iter()
        .map(|x| x.market)
        .collect::<Vec<_>>();

    let mut market_entries = HashMap::new();
    sqlx::query!(
        "
            SELECT
                order_id,
                type_id,
                structure_id,
                virtual_remaining
            FROM market_order_latest mol
            WHERE mol.type_id = ANY($1)
            AND mol.structure_id = ANY($2)
            AND mol.is_buy = false
            AND virtual_remaining > 0
            ORDER BY mol.price ASC
        ",
        &type_ids
            .into_iter()
            .map(|x| *x)
            .collect::<Vec<_>>(),
        &structure_ids
            .iter()
            .map(|x| **x)
            .collect::<Vec<_>>(),
    )
    .fetch_all(pool)
    .await
    .unwrap()
    .into_iter()
    .for_each(|x| {
        let entry = TmpMarketEntry {
            order_id:          x
                .order_id
                .into(),
            virtual_remaining: x.virtual_remaining,
        };

        market_entries
            .entry((x.type_id, x.structure_id))
            .and_modify(|x: &mut Vec<TmpMarketEntry>| x.push(entry.clone()))
            .or_insert(vec![entry]);
    });

    let mut updates = Vec::new();
    for entry in entries {
        let market_entries = if let Some(x) =
            market_entries.get(&(*entry.type_id, *entry.market))
        {
            x
        } else {
            continue;
        };

        let mut remaining = entry.quantity;
        for market_entry in market_entries {
            if remaining == 0 {
                break;
            }

            if market_entry.virtual_remaining < remaining {
                updates.push(Update {
                    order_id:  market_entry.order_id,
                    remaining: 0,
                });
                remaining -= market_entry.virtual_remaining;
            } else if market_entry.virtual_remaining >= remaining {
                updates.push(Update {
                    order_id:  market_entry.order_id,
                    remaining: market_entry.virtual_remaining - remaining,
                });
                remaining = 0;
            }
        }
    }

    sqlx::query!(
        "
            UPDATE market_order_latest AS mol
            SET virtual_remaining = update.remaining
            FROM UNNEST(
                $1::BIGINT[],
                $2::INTEGER[]
            ) as update(order_id, remaining)
            WHERE mol.order_id = update.order_id
        ",
        &updates
            .iter()
            .map(|x| *x.order_id)
            .collect::<Vec<_>>(),
        &updates
            .iter()
            .map(|x| x.remaining)
            .collect::<Vec<_>>(),
    )
    .execute(pool)
    .await
    .unwrap();

    Ok(())
}

#[derive(Clone)]
struct TmpMarketEntry {
    order_id:          OrderId,
    virtual_remaining: i32,
}

struct Update {
    order_id:  OrderId,
    remaining: i32,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, IntoParams)]
pub struct MarketVirtualRequest {
    pub type_id:  TypeId,
    pub market:   StructureId,
    pub quantity: i32,
}
