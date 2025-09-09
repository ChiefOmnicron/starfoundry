use sqlx::PgPool;
use starfoundry_lib_types::CharacterId;
use warp::{Reply, Rejection};

use crate::{ReplyError, Identity};
use super::error::NotificationError;
use super::{Notification, NotificationTarget, NotificationUuid};

pub async fn fetch(
    pool:            &PgPool,
    character_id:    CharacterId,
    notification_id: NotificationUuid,
) -> Result<Notification, NotificationError> {
    let entry = sqlx::query!(r#"
            SELECT
                id,
                target AS "target: NotificationTarget",
                url,
                name
            FROM notification
            WHERE id = $1
            AND owner = $2
        "#,
            *notification_id,
            *character_id,
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| NotificationError::Fetch(
            e,
            notification_id
        ))?;

    if let Some(x) = entry {
        Ok(Notification {
            id:     NotificationUuid(x.id),
            name:   x.name,
            target: x.target,
            url:    x.url,
        })
    } else {
        Err(NotificationError::NotFound(notification_id))
    }
}

/// /notifications/{notificationId}
/// 
#[utoipa::path(
    get,
    operation_id = "notifications_fetch",
    path = "/notifications/{notificationId}",
    tag = "notification",
    params(
        (
            "notificationId" = NotificationUuid,
            description = "UUID of the notification to fetch",
        ),
    ),
    responses(
        (
            body = Notification,
            content_type = "application/json",
            description = "Requested Notification",
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
            description = "Unknown error",
            status = INTERNAL_SERVER_ERROR,
        ),
    ),
)]
pub async fn fetch_api(
    pool:            PgPool,
    identity:        Identity,
    notification_id: NotificationUuid,
) -> Result<impl Reply, Rejection> {
    match fetch(
        &pool,
        identity.character_id(),
        notification_id,
    ).await {
        Ok(x) => Ok(warp::reply::json(&x)),
        Err(NotificationError::NotFound(_)) => {
            tracing::warn!("notification not found {notification_id}");
            Err(ReplyError::Forbidden.into())
        },
        Err(e) => {
            tracing::error!("Unexpected error fetching bpc stock, {e}");
            Err(ReplyError::Internal.into())
        },
    }
}
