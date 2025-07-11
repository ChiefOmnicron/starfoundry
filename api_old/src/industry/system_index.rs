use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use starfoundry_libs_types::SystemId;
use warp::reject::Rejection;
use warp::reply::Reply;

use super::error::IndustryError;
use crate::ReplyError;
use utoipa::{IntoParams, ToSchema};

pub async fn system_index(
    pool:   &PgPool,
    filter: SystemIndexFilter,
) -> Result<Vec<SystemIndex>, IndustryError> {
    sqlx::query!("
            SELECT
                timestamp,
                system_id,
                reaction,
                manufacturing,
                invention,
                copying,
                research_material,
                research_time
            FROM industry_index
            WHERE system_id = $1 AND
                timestamp >= NOW() - INTERVAL '2 MONTHS'
            ORDER BY timestamp DESC
        ",
        *filter.system_id,
    )
    .fetch_all(pool)
    .await
    .map_err(|e| IndustryError::FetchIndustryIndex(e, filter.system_id))
    .map(|x| {
        x.into_iter()
            .map(|y| {
                SystemIndex {
                    timestamp:         y.timestamp.and_utc().timestamp_millis(),
                    system_id:         y.system_id,
                    reaction:          y.reaction,
                    manufacturing:     y.manufacturing,
                    invention:         y.invention,
                    copying:           y.copying,
                    research_material: y.research_material,
                    research_time:     y.research_time,
                }
            })
            .collect::<Vec<_>>()
    })
}

/// /industry/system-index
/// 
#[utoipa::path(
    get,
    operation_id = "industry_system_index",
    path = "/industry/system-index",
    tag = "industry",
    params(SystemIndexFilter),
    responses(
        (
            body = Vec<SystemIndex>,
            content_type = "application/json",
            description = "List of the industry index over the last 6 months",
            status = OK,
        ),
        (
            description = "Invalid parameter",
            status = BAD_REQUEST,
        ),
        (
            description = "Unknown error",
            status = INTERNAL_SERVER_ERROR,
        ),
    ),
)]
pub async fn system_index_api(
    pool:  PgPool,
    query: SystemIndexFilter,
) -> Result<impl Reply, Rejection> {
    match system_index(
        &pool,
        query
    ).await {
        Ok(x) => Ok(warp::reply::json(&x)),
        Err(e) => {
            tracing::error!("Unexpected error fetching structures, {e}");
            Err(ReplyError::Internal.into())
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
#[schema(
    example = json!({
        "timestamp": 1727557959,
        "system_id": 30004759,
        "reaction": 0.0931,
        "manufacturing": 0.1203,
        "invention": 0.2083,
        "copying": 0.1419,
        "research_material": 0.1244,
        "research_time": 0.1296
    })
)]
pub struct SystemIndex {
    /// Timestamp in milliseconds.
    /// Example only in seconds, due to documentation library limitations
    pub timestamp:         i64,
    /// Id of the system the indexes apply to
    pub system_id:         i32,
    pub reaction:          f32,
    pub manufacturing:     f32,
    pub invention:         f32,
    pub copying:           f32,
    pub research_material: f32,
    pub research_time:     f32,
}

#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct SystemIndexFilter {
    #[param(
        example = json!({
            "system_id": 30004759
        }),
        required = true,
    )]
    pub system_id: SystemId,
}
