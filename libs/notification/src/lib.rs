mod discord;
mod error;
mod stock_blueprint;
mod test_message;

pub(crate) use self::discord::*;

pub use self::error::*;
pub use self::stock_blueprint::*;
pub use self::test_message::*;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use starfoundry_libs_eve_api::{CredentialCache, EveApiClient};
use starfoundry_libs_types::CorporationId;
use std::sync::{Arc, Mutex};
use utoipa::ToSchema;

#[async_trait]
pub(crate) trait Notification {
    async fn discord(
        &self,
        url:      String,
        messages: Vec<Discord>,
    ) -> Result<String, Error> {
        if !url.contains("https://discord.com/api") {
            return Err(Error::InvalidTarget("DISCORD".into(), url));
        }

        for message in messages {
            let value = serde_json::to_value(&message)?;

            if let Err(e) = self.json(url.clone(), value).await {
                return Err(e)
            }
        }

        Ok("OK".into())
    }

    async fn json(
        &self,
        url:   String,
        value: serde_json::Value,
    ) -> Result<String, Error> {
        let client = reqwest::Client::new();
        let response = client
            .post(url)
            .json(&value)
            .send()
            .await?;

        if !response.status().is_success() {
            let message = response
                .text()
                .await
                .map_err(Error::GenericReqwestError)?;
            Err(Error::ResponseError(message, value))
        } else {
            let message = response
                .text()
                .await
                .map_err(Error::GenericReqwestError)?;
            Ok(message)
        }
    }
}

pub(crate) async fn api_client(
    corporation_id:   CorporationId,
    credential_cache: Arc<Mutex<CredentialCache>>,
) -> Option<EveApiClient> {
    let cache = {
        credential_cache
            .lock()
            .unwrap()
            .clone()
    };

    if let Ok(client) = cache
        .get((*corporation_id).into())
        .await {
        Some(client)
    } else {
        tracing::warn!(
            "Failed to get valid credentials for {}. Skipping",
            corporation_id
        );
        None
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
