use serde::Serialize;
use sqlx::PgPool;
use utoipa::ToSchema;
use warp::reject::Rejection;
use warp::reply::Reply;

/// /api/v1/appraisal/markets
/// 
/// List of all supported markets
/// 
#[utoipa::path(
    get,
    operation_id = "appraisal_create",
    path = "/api/v1/appraisal/markets",
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
        name: "E3OI-U".into(),
        market_id: 1040278453044,
    });
    markets.push(AppraisalMarket {
        name: "UALX-3".into(),
        market_id: 1046664001931,
    });
    markets.push(AppraisalMarket {
        name: "K7D-II".into(),
        market_id: 1043661023026,
    });

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
