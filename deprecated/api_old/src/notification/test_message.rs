use serde::Deserialize;
use sqlx::PgPool;
use starfoundry_lib_notification::{NotificationTarget, TestMessage};
use utoipa::ToSchema;
use warp::{Reply, Rejection};

use crate::{BadRequestPayload, Identity, ReplyError};

pub async fn test_message(
    test_message_data: NotificationTestMessage,
) -> Result<String, String> {
    TestMessage::new()
        .send(
            test_message_data.target,
            test_message_data.url,
        )
        .await
}

/// /notifications/test-message
/// 
#[utoipa::path(
    post,
    operation_id = "notification_test_message",
    path = "/notifications/test-message",
    tag = "notification",
    responses(
        (
            body = String,
            content_type = "application/json",
            description = "Response from the server",
            status = OK,
        ),
        (
            description = "The other side responded with an error",
            status = BAD_REQUEST,
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
pub async fn test_message_api(
    _pool:             PgPool,
    _identity:         Identity,
    test_message_data: NotificationTestMessage,
) -> Result<impl Reply, Rejection> {
    match test_message(
        test_message_data,
    ).await {
        Ok(x) => Ok(warp::reply::json(&x)),
        Err(e) => Err(ReplyError::BadRequestWithPayload(BadRequestPayload {
            error: "FAILED_MESSAGE".into(),
            description: e,
        }).into()),
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct NotificationTestMessage {
    pub target: NotificationTarget,
    pub url:    String,
}
