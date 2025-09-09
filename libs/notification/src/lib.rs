mod discord;
mod error;
mod test_message;

pub use self::discord::*;
pub use self::error::*;
pub use self::test_message::*;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub async fn send_json_notification<S: Into<String>>(
    url:   S,
    value: serde_json::Value,
) -> Result<String, Error> {
    let client = reqwest::Client::new();
    let response = client
        .post(url.into())
        .json(&value)
        .send()
        .await?;

    if !response.status().is_success() {
        let message = response
            .text()
            .await?;
        Err(Error::ResponseError(message, value))
    } else {
        let message = response
            .text()
            .await?;
        Ok(message)
    }
}

#[derive(Debug, Deserialize, Serialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "NOTIFICATION_TARGET")]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NotificationTarget {
    Discord,
    Json,
}
