use utoipa::OpenApi;
use serde::Serialize;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "StarFoundry API",
        description = include_str!("api_doc_industry.md"),
        contact(
            url = "https://github.com/ChiefOmnicron/starfoundry"
        ),
        license(
            name = "Dual licensed under Apache-2.0 and MIT"
        ),
    ),
)]
pub struct ApiDoc;

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

/// Use in error types
#[derive(Serialize)]
pub struct ErrorResponse {
    /// General error name
    pub error: String,
    /// Human description of the error
    pub description: String,
}

/*
/// the operation was successful, but does not return any data
#[derive(utoipa::IntoResponses)]
#[response(status = NO_CONTENT)]
pub struct NoContent;

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
    pub error: BadRequestError,
    /// Human description of the error
    pub description: String,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "UPPERCASE")]
pub enum BadRequestError {
    Deserialization,
    Validation,
}

/// the resource was not found
#[derive(utoipa::IntoResponses)]
#[response(status = NOT_FOUND)]
#[response(
    status = NOT_FOUND,
    example = json!({
        "error": "NOT_FOUND",
        "description": "Authenticate and try again"
    })
)]
pub struct NotFound {
    /// General error name
    pub error: String,
    /// Human description of the error
    pub description: String,
}

/// authenticate and then try again
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

/// you are not allowed to see the resource
#[derive(utoipa::IntoResponses)]
#[response(status = FORBIDDEN)]
#[response(
    status = FORBIDDEN,
    example = json!({
        "error": "FORBIDDEN",
        "description": "You are not authorized to see this resource"
    })
)]
pub struct Forbidden {
    /// General error name
    pub error: String,
    /// Human description of the error
    pub description: String,
}

/// wrong media type, must be application/json
#[derive(utoipa::IntoResponses)]
#[response(status = UNSUPPORTED_MEDIA_TYPE)]
pub struct UnsupportedMediaType;
 */
