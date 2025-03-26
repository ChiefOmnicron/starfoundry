use sqlx::PgPool;
use starfoundry_libs_types::CharacterId;
use warp::http::StatusCode;
use warp::reject::Rejection;
use warp::reply::Reply;

use crate::{Identity, ReplyError};
use super::{NotificationError, NotificationUuid};

pub async fn delete(
    pool:            &PgPool,
    character_id:    CharacterId,
    notification_id: NotificationUuid,
) -> Result<(), NotificationError> {
    let result = sqlx::query!("
            DELETE FROM notifications
            WHERE id = $1
            AND owner = $2
        ",
            *notification_id,
            *character_id,
        )
        .execute(pool)
        .await
        .map_err(|e| NotificationError::Delete(e, notification_id))?;

    if result.rows_affected() > 0 {
        Ok(())
    } else {
        Err(NotificationError::NotFound(notification_id))
    }
}

#[utoipa::path(
    delete,
    operation_id = "notifications_delete",
    path = "/api/v1/notifications/{notificationId}",
    tag = "notification",
    params(
        (
            "notificationId" = NotificationUuid,
            description = "UUID of the notification to delete",
        ),
    ),
    responses(
        (
            description = "The notification was deleted",
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
            description = "Unknown error",
            status = INTERNAL_SERVER_ERROR,
        ),
    ),
)]
pub async fn delete_api(
    pool:            PgPool,
    identity:        Identity,
    notification_id: NotificationUuid,
) -> Result<impl Reply, Rejection> {
    match delete(
        &pool,
        identity.character_id(),
        notification_id,
    ).await {
        Ok(_) => Ok(warp::reply::with_status(
            warp::reply::json(&()),
            StatusCode::NO_CONTENT,
        )),
        Err(NotificationError::NotFound(_)) => {
            Err(ReplyError::Forbidden.into())
        },
        Err(e) => {
            tracing::error!("Unexpected error deleting bpc stock, {e}");
            Err(ReplyError::Internal.into())
        },
    }
}
