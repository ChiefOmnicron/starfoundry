use serde::Deserialize;
use sqlx::PgPool;
use starfoundry_libs_appraisal::internal::{Appraisal, AppraisalOptions};
use utoipa::ToSchema;
use warp::reject::Rejection;
use warp::reply::Reply;

use crate::ReplyError;
use crate::api_docs::{BadRequest, InternalServerError, UnsupportedMediaType};
use starfoundry_libs_appraisal::Persistance;
use crate::metric::{RequestStatus, WithMetric};

/// /appraisal
/// 
/// Creates a new appraisal
/// 
#[utoipa::path(
    post,
    operation_id = "appraisal_create",
    path = "/appraisal",
    tag = "appraisal",
    request_body(
        content = AppraisalCreateBody,
        description = "New appraisal information",
        content_type = "application/json"
    ),
    responses(
        (
            body = Appraisal,
            content_type = "application/json",
            description = "ID of the new project",
            status = CREATED,
        ),
        BadRequest,
        UnsupportedMediaType,
        InternalServerError,
    ),
)]
pub async fn create(
    pool:   PgPool,
    metric: WithMetric,
    body:   AppraisalCreateBody,
) -> Result<impl Reply, Rejection> {
    let mut options = AppraisalOptions::default();
    options.set_store(body.store);
    options.set_market_id(body.market_id);
    options.set_price_modifier(body.price_modifier);
    options.set_comment(body.comment);

    match starfoundry_libs_appraisal::internal::create_raw(
        &pool,
        body.appraisal,
        Some(options)
    ).await {
        Ok(x)  => {
            metric.inc_appraisal_created_count(RequestStatus::Ok);
            Ok(warp::reply::json(&x))
        },
        Err(e) => {
            tracing::error!("{}", e);
            metric.inc_appraisal_created_count(RequestStatus::Error);
            Err(ReplyError::Internal.into())
        },
    }
}

#[derive(Debug, Deserialize, ToSchema)]
#[schema(
    example = json!({
        "appraisal": "raw appraisal string, items must be separated by new lines",
        "comment": "this is a cool comment",
        "market": 60003760,
        "price_modifier": 100,
        "store": true
    })
)]
pub struct AppraisalCreateBody {
    /// raw entry of the items that should be appraised
    pub appraisal:      String,
    /// whether or not the apprisal should be stored, true per default
    #[serde(default, deserialize_with = "Persistance::deserialize")]
    pub store:          Option<Persistance>,
    /// market that should be used, jita is the default
    pub market_id:      Option<i64>,
    /// modifier for the price, default is 100%
    pub price_modifier: Option<i16>,
    /// comment for the appraisal, per default empty
    pub comment:        Option<String>,
}
