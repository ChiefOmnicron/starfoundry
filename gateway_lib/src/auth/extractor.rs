use axum::extract::{FromRequestParts, OptionalFromRequestParts};
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::Json;
use serde_json::json;
use starfoundry_lib_types::{AllianceId, CharacterId, CorporationId};

use crate::{Error, HEADER_ALLIANCE_ID, HEADER_CHARACTER_ID, HEADER_CORPORATION_ID, HEADER_IS_ADMIN, HEADER_SERVICE, HEADER_SOURCE, Result};

/// Extracts the identity of the requester.
/// 
#[derive(Debug)]
pub struct ExtractIdentity {
    pub character_id:   CharacterId,
    pub corporation_id: CorporationId,
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
            Err(Error::Unauthorized)
        }
    }
}

impl<S> FromRequestParts<S> for ExtractIdentity
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, Json<serde_json::Value>);

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let host = parts.headers
            .get(HEADER_SOURCE)
            .map(|x| x.to_str().unwrap_or_default().into());

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
            tracing::error!("{HEADER_CHARACTER_ID} header not present");
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "error": "UNAUTHORIZED",
                    "description": "Authenticate and try again"
                }))
            ));
        };

        let corporation_id = if let Some(x) = parts.headers.get(HEADER_CORPORATION_ID) {
            if let Ok(x) = x.to_str()
                .unwrap_or_default()
                .parse::<CorporationId>() {
                x
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
            tracing::error!("{HEADER_CORPORATION_ID} header not present");
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "error": "UNAUTHORIZED",
                    "description": "Authenticate and try again"
                }))
            ));
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
            x.to_str()
                .unwrap_or_default()
                .parse::<i32>()
                .unwrap_or(0) == 1
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

impl<S> OptionalFromRequestParts<S> for ExtractIdentity
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, Json<serde_json::Value>);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> std::result::Result<Option<Self>, Self::Rejection> {
        if let Ok(x) = <ExtractIdentity as FromRequestParts<S>>::from_request_parts(parts, state).await {
            Ok(Some(x))
        } else {
            Ok(None)
        }
    }
}
