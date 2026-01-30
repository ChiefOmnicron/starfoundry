use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::Json;
use reqwest::header::HOST;
use serde_json::json;
use starfoundry_lib_types::{AllianceId, CharacterId, CorporationId};
use crate::{Error, Result};

pub const HEADER_CHARACTER_ID: &str   = "X-SF-CharacterId";
pub const HEADER_CORPORATION_ID: &str = "X-SF-CorporationId";
pub const HEADER_ALLIANCE_ID: &str    = "X-SF-AllianceId";
pub const HEADER_IS_ADMIN: &str       = "X-SF-IsAdmin";
pub const HEADER_SERVICE: &str        = "X-SF-Service";

/// Extracts the identity of the requester.
/// 
#[derive(Debug)]
pub struct ExtractIdentity {
    pub character_id:   CharacterId,
    pub corporation_id: Option<CorporationId>,
    pub alliance_id:    Option<AllianceId>,
    pub is_admin:       bool,
    pub service_name:   String,

    /// The host is usually set by the gateway service, and taken directly from
    /// the existing HOST header
    /// For request that needs valid user credentials, this parameter is required
    host:               Option<String>,
}

impl ExtractIdentity {
    pub fn host(
        &self,
    ) -> Result<String> {
        if let Some(x) = self.host.clone() {
            Ok(x)
        } else {
            return Err(Error::Unauthorized)
        }
    }
}

impl<S> FromRequestParts<S> for ExtractIdentity
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, Json<serde_json::Value>);

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let host = if let Some(x) = parts.headers.get(HOST) {
            Some(x.to_str().unwrap_or_default().into())
        } else {
            None
        };

        let service_name = if let Some(x) = parts.headers.get(HEADER_SERVICE) {
            x.to_str().unwrap_or_default().into()
        } else {
            tracing::error!("Parsing {HEADER_SERVICE} header failed");
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "error": "UNAUTHORIZED",
                    "description": "Not all headers are set"
                }))
            ))
        };

        let character_id = if let Some(x) = parts.headers.get(HEADER_CHARACTER_ID) {
            if let Ok(x) = x.to_str()
                .unwrap_or_default()
                .parse::<CharacterId>() {
                x
            } else {
                tracing::error!("Parsing {HEADER_CHARACTER_ID} header failed");
                return Err((
                    StatusCode::UNAUTHORIZED,
                    Json(json!({
                        "error": "UNAUTHORIZED",
                        "description": "Authenticate and try again"
                    }))
                ))
            }
        } else {
            // TODO: remove when refactoring is done
            if service_name == "SF_MARKET_WORKER" {
                CharacterId(0)
            } else {
                tracing::error!("{HEADER_CHARACTER_ID} header not present");
                return Err((
                    StatusCode::UNAUTHORIZED,
                    Json(json!({
                        "error": "UNAUTHORIZED",
                        "description": "Authenticate and try again"
                    }))
                ))
            }
        };

        let corporation_id = if let Some(x) = parts.headers.get(HEADER_CORPORATION_ID) {
            if let Ok(x) = x.to_str()
                .unwrap_or_default()
                .parse::<CorporationId>() {
                Some(x)
            } else {
                tracing::error!("Parsing {HEADER_CORPORATION_ID} header failed");
                return Err((
                    StatusCode::UNAUTHORIZED,
                    Json(json!({
                        "error": "UNAUTHORIZED",
                        "description": "Authenticate and try again"
                    }))
                ))
            }
        } else {
            None
        };

        let alliance_id = if let Some(x) = parts.headers.get(HEADER_ALLIANCE_ID) {
            if let Ok(x) = x.to_str()
                .unwrap_or_default()
                .parse::<AllianceId>() {
                Some(x)
            } else {
                tracing::error!("Parsing {HEADER_ALLIANCE_ID} header failed");
                return Err((
                    StatusCode::UNAUTHORIZED,
                    Json(json!({
                        "error": "UNAUTHORIZED",
                        "description": "Authenticate and try again"
                    }))
                ))
            }
        } else {
            None
        };

        let is_admin = if let Some(x) = parts.headers.get(HEADER_IS_ADMIN) {
            let parsed = x.to_str()
                .unwrap_or_default()
                .parse::<i32>()
                .unwrap_or(0);
            if parsed == 1 {
                true
            } else {
                false
            }
        } else {
            false
        };

        Ok(ExtractIdentity {
            host,
            character_id,
            corporation_id,
            alliance_id,
            is_admin,
            service_name,
        })
    }
}
