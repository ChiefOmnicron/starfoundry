use serde::Deserialize;
use sqlx::PgPool;
use starfoundry_lib_types::CharacterId;
use utoipa::ToSchema;
use warp::http::StatusCode;
use warp::reject::Rejection;
use warp::reply::Reply;

use crate::{Identity, ReplyError};
use super::{NotificationError, NotificationUuid, NotificationTarget};

pub async fn update(
    pool:            &PgPool,
    character_id:    CharacterId,
    notification_id: NotificationUuid,
    notification:    UpdateNotification,
) -> Result<(), NotificationError> {
    let result = sqlx::query!("
            UPDATE notification
            SET
                target = $3,
                url = $4,
                name = $5
            WHERE id = $1
            AND owner = $2
        ",
            *notification_id,
            *character_id,
            notification.target as _,
            notification.url,
            notification.name,
        )
        .execute(pool)
        .await
        .map_err(|e| NotificationError::Update(e, notification_id))?;

    if result.rows_affected() > 0 {
        Ok(())
    } else {
        Err(NotificationError::NotFound(notification_id))
    }
}

/// /notifications/{notificationId}
/// 
#[utoipa::path(
    put,
    operation_id = "notifications_update",
    path = "/notifications/{notificationId}",
    tag = "notification",
    params(
        (
            "notificationId" = NotificationUuid,
            description = "UUID of the notification to delete",
        ),
    ),
    request_body = UpdateNotification,
    responses(
        (
            description = "Entry udpated",
            status = NO_CONTENT,
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
pub async fn update_api(
    pool:            PgPool,
    identity:        Identity,
    notification_id: NotificationUuid,
    notification:    UpdateNotification,
) -> Result<impl Reply, Rejection> {
    match update(
        &pool,
        identity.character_id(),
        notification_id,
        notification,
    ).await {
        Ok(_) => Ok(warp::reply::with_status(
            warp::reply::json(&()),
            StatusCode::NO_CONTENT,
        )),
        Err(NotificationError::NotFound(_)) => {
            Err(ReplyError::Forbidden.into())
        },
        Err(e) => {
            tracing::error!("Unexpected error updating notification, {e}");
            Err(ReplyError::Internal.into())
        },
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateNotification {
    /// target for the notification
    #[schema(
        example = json!("DISCORD")
    )]
    pub target: NotificationTarget,
    /// target url for the notification
    pub url:    String,
    /// name of the notification
    pub name:   String,
}
