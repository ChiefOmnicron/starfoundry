use serde::Deserialize;
use sqlx::PgPool;
use starfoundry_libs_types::CharacterId;
use utoipa::ToSchema;
use warp::reject::Rejection;
use warp::reply::Reply;

use crate::{Identity, ReplyError};
use super::error::NotificationError;
use super::{NotificationTarget, NotificationUuid};

pub async fn create(
    pool:         &PgPool,
    character_id: CharacterId,
    notification: CreateNotification,
) -> Result<NotificationUuid, NotificationError> {
    sqlx::query!("
            INSERT INTO notification(
                target,
                url,
                name,
                owner
            )
            VALUES ($1, $2, $3, $4)
            RETURNING id
        ",
            notification.target as _,
            notification.url,
            notification.name,
            *character_id,
        )
        .fetch_one(pool)
        .await
        .map(|x| {
            NotificationUuid(x.id)
        })
        .map_err(NotificationError::Create)
}

/// /notifications
/// 
#[utoipa::path(
    post,
    operation_id = "notifications_create",
    path = "/notifications",
    tag = "notification",
    request_body = CreateNotification,
    responses(
        (
            body = NotificationUuid,
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
pub async fn create_api(
    pool:         PgPool,
    identity:     Identity,
    notification: CreateNotification,
) -> Result<impl Reply, Rejection> {
    match create(
        &pool,
        identity.character_id(),
        notification,
    ).await {
        Ok(x) => Ok(warp::reply::json(&x)),
        Err(e) => {
            tracing::error!("Unexpected error creating bpc stock, {e}");
            Err(ReplyError::Internal.into())
        },
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateNotification {
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
