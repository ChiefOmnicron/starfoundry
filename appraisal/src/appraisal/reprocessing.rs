use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use sqlx::PgPool;
use starfoundry_lib_appraisal::{Appraisal, AppraisalCode, AppraisalMode, AppraisalTotal};
use starfoundry_lib_market::MarketBulkResponse;

use crate::api_docs::{Forbidden, InternalServerError, NotFound, Unauthorized};
use crate::state::AppState;
use crate::appraisal::{AppraisalError, Result};

/// Fetch Appraisal Reprocessing
/// 
/// - Alternative route: `/latest/appraisals/{AppraisalCode}/reprocessing`
/// - Alternative route: `/v1/appraisals/{AppraisalCode}/reprocessing`
/// 
/// ---
/// 
/// Fetches the appraisal
/// 
#[utoipa::path(
    get,
    path = "/{ProjectUuid}/reprocessing",
    tag = "Appraisals",
    params(
        AppraisalCode,
    ),
    responses(
        (
            body = Option<Appraisal>,
            description = "Information about the appraisal",
            status = OK,
        ),
        Unauthorized,
        Forbidden,
        NotFound,
        InternalServerError,
    ),
    security(
        ("api_key" = [])
    ),
)]
pub async fn api(
    State(state):   State<AppState>,
    Path(code):     Path<AppraisalCode>
) -> Result<impl IntoResponse> {
    let entry = fetch(
            &state.postgres,
            code,
        )
        .await?;

    if let Some(x) = entry {
        Ok(
            (
                StatusCode::OK,
                Json(x)
            )
            .into_response()
        )
    } else {
        Ok(
            (
                StatusCode::NOT_FOUND,
                Json(())
            )
            .into_response()
        )
    }
}

async fn fetch(
    pool:   &PgPool,
    code:   AppraisalCode,
) -> Result<Option<Appraisal>> {
    let info = sqlx::query!("
            SELECT
                code,
                market_id,
                price_modifier,
                market_info,
                invalid_items,
                comment,
                raw,
                mode,
                created_at
            FROM appraisal
            WHERE code = $1
        ",
            &*code,
        )
        .fetch_optional(pool)
        .await?;

    if let Some(x) = info {
        let market_data: Vec<MarketBulkResponse> = serde_json::from_value(x.market_info)
            .map_err(AppraisalError::ParseMarketData)?;

        let total_buy = market_data
            .iter()
            .filter(|x| x.buy_price.is_some())
            .map(|x| x.buy_price.as_ref().map(|y| y.max * x.quantity as f64).unwrap_or_default())
            .sum();
        let total_sell = market_data
            .iter()
            .filter(|x| x.sell_price.is_some())
            .map(|x| x.sell_price.as_ref().map(|y| y.min * x.quantity as f64).unwrap_or_default())
            .sum();
        let total = AppraisalTotal {
            buy:    total_buy,
            sell:   total_sell,
        };

        let invalid_items: Vec<String> = x
            .invalid_items
            .map(|y| y.lines().map(|z| z.to_string()).collect::<Vec<_>>())
            .unwrap_or_default();

        let mode = AppraisalMode::try_from(x.mode)
            .unwrap_or(AppraisalMode::Appraisal);

        Ok(Some(Appraisal {
            code:           code,
            invalid:        invalid_items,
            items:          market_data,
            comment:        x.comment,
            market_id:      x.market_id.into(),
            created_at_ts:  x.created_at.timestamp(),
            mode:           mode,
            modifier:       x.price_modifier as u32,
            raw:            x.raw,
            total:          total,
        }))
    } else {
        Ok(None)
    }
}
