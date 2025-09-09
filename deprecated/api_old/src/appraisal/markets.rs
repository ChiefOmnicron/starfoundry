use serde::Serialize;
use sqlx::PgPool;
use utoipa::ToSchema;
use warp::reject::Rejection;
use warp::reply::Reply;

use crate::metric::{RequestStatus, WithMetric};

/// /appraisals/markets
/// 
/// List of all supported markets
/// 
#[utoipa::path(
    get,
    operation_id = "appraisal_create",
    path = "/appraisals/markets",
    tag = "appraisal",
    responses(
        (
            body = AppraisalMarket,
            content_type = "application/json",
            description = "List of all supported markets",
            status = OK,
        ),
    ),
)]
pub async fn markets(
    _: PgPool,
    metric: WithMetric,
) -> Result<impl Reply, Rejection> {
    let mut markets = Vec::new();

    markets.push(AppraisalMarket {
        name: "Jita 4-4".into(),
        market_id: 60003760,
    });
    markets.push(AppraisalMarket {
        name: "Amarr".into(),
        market_id: 60008494,
    });
    markets.push(AppraisalMarket {
        name: "UALX-3".into(),
        market_id: 1046664001931,
    });
    markets.push(AppraisalMarket {
        name: "C-J6MT".into(),
        market_id: 1049588174021,
    });

    metric.inc_appraisal_market_count(RequestStatus::Ok);
    Ok(warp::reply::json(&markets))
}

#[derive(Debug, Serialize, ToSchema)]
#[schema(
    example = json!([{
        "name": "Jita 4-4",
        "market_id": 60003760
    }, {
        "name": "Amarr",
        "market_id": 60008494
    }])
)]
pub struct AppraisalMarket {
    pub name:      String,
    pub market_id: i64,
}
