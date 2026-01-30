use axum::extract::rejection::JsonRejection;
use axum::http::header::AUTHORIZATION;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Serialize;
use utoipa::{Modify, OpenApi};
use utoipa::openapi::security::{ApiKey, ApiKeyValue, SecurityScheme};
use axum::Json;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "StarFoundry API",
        description = include_str!("api_doc.md"),
        contact(
            url = "https://github.com/ChiefOmnicron/starfoundry"
        ),
        license(
            name = "Dual licensed under Apache-2.0 and MIT"
        ),
    ),
    modifiers(&SecurityAddon),
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "api_key",
                SecurityScheme::ApiKey(
                    ApiKey::Header(ApiKeyValue::new(AUTHORIZATION.to_string()))
                ),
            )
        }
    }
}

/// the given parameters are incorrect
#[allow(dead_code)]
#[derive(utoipa::IntoResponses)]
#[response(
    status = BAD_REQUEST,
    example = json!({
        "error": "DESERIALIZATION_ERROR",
        "description": "The body could not be parsed, make sure it's valid json and validate the routes requires parameters"
    })
)]
pub struct BadRequest {
    /// General error name
    pub error: String,
    /// Human description of the error
    pub description: String,
}

/// authenticate and then try again
#[allow(dead_code)]
#[derive(utoipa::IntoResponses)]
#[response(status = UNAUTHORIZED)]
#[response(
    status = UNAUTHORIZED,
    example = json!({
        "error": "UNAUTHORIZED",
        "description": "Authenticate and try again"
    })
)]
pub struct Unauthorized {
    /// General error name
    pub error: String,
    /// Human description of the error
    pub description: String,
}

/// you are not allowed to see this resource
#[allow(dead_code)]
#[derive(utoipa::IntoResponses)]
#[response(status = FORBIDDEN)]
#[response(
    status = FORBIDDEN,
    example = json!({
        "error": "FORBIDDEN",
        "description": "Get good and try again"
    })
)]
pub struct Forbidden {
    /// General error name
    pub error: String,
    /// Human description of the error
    pub description: String,
}

/// no resource found
#[allow(dead_code)]
#[derive(utoipa::IntoResponses)]
#[response(status = NOT_FOUND)]
#[response(
    status = NOT_FOUND,
    example = json!({
        "error": "NOT_FOUND",
        "description": "No resource found"
    })
)]
pub struct NotFound {
    /// General error name
    pub error: String,
    /// Human description of the error
    pub description: String,
}

/// there was an unknown error
#[allow(dead_code)]
#[derive(utoipa::IntoResponses)]
#[response(
    status = INTERNAL_SERVER_ERROR,
    example = json!({
        "error": "UNKNOWN",
        "description": "An unknown error occurred"
    })
)]
pub struct InternalServerError {
    /// General error name
    pub error: String,
    /// Human description of the error
    pub description: String,
}

/// authenticate and then try again
#[allow(dead_code)]
#[derive(utoipa::IntoResponses)]
#[response(status = UNSUPPORTED_MEDIA_TYPE)]
#[response(
    status = UNSUPPORTED_MEDIA_TYPE,
    example = json!({
        "error": "UNSUPPORTED_MEDIA_TYPE",
        "description": "The datatype is invalid. Try application/json"
    })
)]
pub struct UnsupportedMediaType {
    /// General error name
    pub error: String,
    /// Human description of the error
    pub description: String,
}

/// authenticate and then try again
#[allow(dead_code)]
#[derive(utoipa::IntoResponses)]
#[response(status = UNPROCESSABLE_ENTITY)]
#[response(
    status = UNPROCESSABLE_ENTITY,
    example = json!({
        "error": "UNPROCESSABLE_ENTITY",
        "description": "The given data is not valid"
    })
)]
pub struct UnprocessableEntity {
    /// General error name
    pub error: String,
    /// Human description of the error
    pub description: String,
}

/// Use in error types
#[derive(Serialize)]
pub struct ErrorResponse {
    /// General error name
    pub error: String,
    /// Human description of the error
    pub description: String,
}

pub fn format_json_errors(
    error: JsonRejection,
) -> impl IntoResponse {
    match error {
        JsonRejection::JsonDataError(_) => {
            (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(
                    ErrorResponse {
                        error: "UNPROCESSABLE_ENTITY".into(),
                        description: "Make sure that all required fields are set".into(),
                    }
                )
            )
        },
        JsonRejection::JsonSyntaxError(_) => {
            (
                StatusCode::BAD_REQUEST,
                Json(
                    ErrorResponse {
                        error: "BAD_REQUEST".into(),
                        description: "Validate that the data you send is valid".into(),
                    }
                )
            )
        },
        JsonRejection::MissingJsonContentType(_) => {
            (
                StatusCode::UNSUPPORTED_MEDIA_TYPE,
                Json(
                    ErrorResponse {
                        error: "UNSUPPORTED_MEDIA_TYPE".into(),
                        description: "The datatype is invalid. Try application/json".into(),
                    }
                )
            )
        },
        _ => {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(
                    ErrorResponse {
                        error: "UNKNOWN".into(),
                        description: "An unknown error occurred, please try again later.".into(),
                    }
                )
            )
        }
    }
}
