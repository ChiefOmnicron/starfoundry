use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::Json;
use serde_json::json;
use starfoundry_lib_types::{AllianceId, CharacterId, CorporationId};

const HEADER_CHARACTER_ID: &str   = "X-SF-CharacterId";
const HEADER_CORPORATION_ID: &str = "X-SF-CorporationId";
const HEADER_ALLIANCE_ID: &str    = "X-SF-AllianceId";
const HEADER_IS_ADMIN: &str       = "X-SF-IsAdmin";

#[derive(Debug)]
pub struct ExtractIdentity {
    pub character_id:   CharacterId,
    pub corporation_id: CorporationId,
    pub alliance_id:    Option<AllianceId>,
    pub is_admin:       bool,
}

impl<S> FromRequestParts<S> for ExtractIdentity
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, Json<serde_json::Value>);

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
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
            ))
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
            ))
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
            character_id,
            corporation_id,
            alliance_id,
            is_admin,
        })
    }
}
