use serde::Deserialize;
use sqlx::PgPool;
use starfoundry_libs_types::TypeId;
use utoipa::ToSchema;
use warp::reject::Rejection;
use warp::reply::Reply;

use crate::{Identity, ReplyError};
use super::{BlueprintStockError, BlueprintStockUuid};

pub async fn add_threshold(
    pool:               &PgPool,
    blueprint_stock_id: BlueprintStockUuid,
    thresholds:         Vec<AddThreshold>,
) -> Result<(), BlueprintStockError> {
    let result = sqlx::query!("
            SELECT id
            FROM stock_blueprints
            WHERE id = $1
        ",
            *blueprint_stock_id
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| BlueprintStockError::FetchById(e, blueprint_stock_id))?;

    if result.is_none() {
        return Err(BlueprintStockError::NotFound(blueprint_stock_id));
    }

    let mut type_ids = Vec::new();
    let mut want = Vec::new();
    let mut critical = Vec::new();
    let mut min_runs = Vec::new();
    let mut min_me = Vec::new();
    let mut min_te = Vec::new();

    for threshold in thresholds {
        type_ids.push(*threshold.type_id);
        want.push(threshold.want);
        critical.push(threshold.critical);
        min_runs.push(threshold.min_runs);
        min_me.push(threshold.min_me);
        min_te.push(threshold.min_te);
    }

    sqlx::query!("
            INSERT INTO stock_blueprint_thresholds (
                blueprint_stock_id,
                type_id,
                want,
                critical,
                min_runs,
                min_me,
                min_te
            )
            SELECT $1, * FROM UNNEST(
                $2::INTEGER[],
                $3::INTEGER[],
                $4::INTEGER[],
                $5::INTEGER[],
                $6::INTEGER[],
                $7::INTEGER[]
            )
        ",
            *blueprint_stock_id,
            &type_ids,
            &want,
            &critical,
            &min_runs,
            &min_me,
            &min_te
        )
        .fetch_one(pool)
        .await
        .map(drop)
        .map_err(|e| BlueprintStockError::AddThreshold(e, blueprint_stock_id))
}

/// /stocks/blueprints/{blueprintStockId}/thresholds
/// 
#[utoipa::path(
    post,
    operation_id = "stocks_blueprints_thresholds_add",
    path = "/stocks/blueprints/{blueprintStockId}/thresholds",
    tag = "blueprint-stocks",
    params(
        (
            "blueprintStockId" = Vec<BlueprintStockUuid>,
            description = "UUID of the bpc stock to fetch",
        ),
    ),
    request_body = Vec<AddThreshold>,
    responses(
        (
            body = BlueprintStockUuid,
            content_type = "application/json",
            description = "UUID of the new entry",
            status = OK,
        ),
        (
            description = "Invalid parameter",
            status = BAD_REQUEST,
        ),
        (
            description = "Not Found",
            status = NOT_FOUND,
        ),
        (
            description = "The requester is not authenticated",
            status = UNAUTHORIZED,
        ),
        (
            description = "Only application/json is supported",
            status = UNSUPPORTED_MEDIA_TYPE,
        ),
        (
            description = "Unknown error",
            status = INTERNAL_SERVER_ERROR,
        ),
    ),
)]
pub async fn add_threshold_api(
    pool:               PgPool,
    _:                  Identity,
    blueprint_stock_id: BlueprintStockUuid,
    thresholds:         Vec<AddThreshold>,
) -> Result<impl Reply, Rejection> {
    match add_threshold(
        &pool,
        blueprint_stock_id,
        thresholds,
    ).await {
        Ok(x) => Ok(warp::reply::json(&x)),
        Err(BlueprintStockError::NotFound(_)) => {
            Err(ReplyError::Forbidden.into())
        },
        Err(e) => {
            tracing::error!("Unexpected error creating bpc stock, {e}");
            Err(ReplyError::Internal.into())
        },
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct AddThreshold {
    pub type_id:  TypeId,
    /// how many blueprints are at least wanted
    pub want:     i32,
    /// below this point they definitly need to be stocked back up
    #[serde(default)]
    pub critical: i32,
    /// minimum number of runs the blueprint needs to have
    /// default: 0
    #[serde(default)]
    pub min_runs: i32,
    /// minimum material efficiency the blueprint needs to have
    /// default: 0
    #[serde(default)]
    pub min_me:   i32,
    /// minimum time efficiency the blueprint needs to have
    /// default: 0
    #[serde(default)]
    pub min_te:   i32,
}
