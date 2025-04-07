use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use starfoundry_libs_types::CharacterId;
use utoipa::ToSchema;
use warp::{Reply, Rejection};

use crate::{Identity, ReplyError};
use super::{NotificationError, NotificationUuid};

pub async fn list(
    pool:         &PgPool,
    character_id: CharacterId,
    filter:       NotificationFilter,
) -> Result<Vec<NotificationUuid>, NotificationError> {
    let filter_status: Vec<String> = if filter.status.is_empty() {
        Vec::new()
    } else {
        filter.status
            .split(",")
            .map(|x| x.into())
            .collect::<Vec<_>>()
    };

    sqlx::query!(r#"
            SELECT id
            FROM notifications
            WHERE
                (
                    NOT (LOWER(name) LIKE '%' || LOWER($2) || '%') IS FALSE AND
                    NOT (target = ANY($3::NOTIFICATION_TARGET[])) IS FALSE
                )
                AND
                (
                    owner = $1
                )
            ORDER BY name
        "#,
            *character_id,
            &filter.name,
            &filter_status as _,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| NotificationError::List(e, character_id, filter))
        .map(|x| {
            x.into_iter()
                .map(|y| NotificationUuid::new(y.id))
                .collect::<Vec<_>>()
        })
}

/// /notifications
/// 
#[utoipa::path(
    get,
    operation_id = "notification_list",
    path = "/notifications",
    tag = "notification",
    params(
        (
            "filter" = NotificationFilter,
            Query,
            description = "Filters the notifications"
        )
    ),
    responses(
        (
            body = Vec<Uuid>,
            content_type = "application/json",
            description = "List of all Notifications that match the filter",
            status = OK,
        ),
        (
            description = "Invalid parameter",
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
pub async fn list_api(
    pool:     PgPool,
    identity: Identity,
    filter:   NotificationFilter,
) -> Result<impl Reply, Rejection> {
    match list(
        &pool,
        identity.character_id(),
        filter,
    ).await {
        Ok(x) => Ok(warp::reply::json(&x)),
        Err(e) => {
            tracing::error!("Unexpected error listing structures, {e}");
            Err(ReplyError::Internal.into())
        },
    }
}


#[derive(Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct NotificationFilter {
    #[serde(default)]
    pub name:   String,

    // workourd as arrays aren´t supported
    #[serde(default = "default_target")]
    pub status: String,
}

fn default_target() -> String {
    "DISCORD,JSON".into()
}
