use axum::extract::Query;
use axum::http::header::{LOCATION, SET_COOKIE};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use std::collections::HashMap;

use crate::api_docs::{BadRequest, InternalServerError};

/// Callback
/// 
/// Alternative route: `/latest/auth/callback`
/// Alternative route: `/v1/auth/callback`
/// 
/// ---
/// 
/// Called from the Eve-Gateway service to finish authentication
/// 
#[utoipa::path(
    get,
    path = "/callback",
    tag = "Auth",
    responses(
        (
            status = TEMPORARY_REDIRECT,
            description = "Redirects back to the app",
            body = String,
            content_type = "text/plain",
            example = json!("https://industry.starfoundry.space/")
        ),
        BadRequest,
        InternalServerError,
    ),
)]
// TODO: move into eve_gateway_lib
pub async fn callback(
    Query(params): Query<HashMap<String, String>>
) -> impl IntoResponse {
    let refresh_token = if let Some(x) = params.get("refresh_token") {
        x
    } else {
        return (
            StatusCode::BAD_REQUEST,
        ).into_response()
    };

    (
        StatusCode::FOUND,
        [(
            LOCATION,
            "https://industry.dev.starfoundry.space/project-groups".to_string(),
        ), (
            SET_COOKIE,
            (&format!("refresh_token={}; HttpOnly; Secure; SameSite=Strict; Path=/api/auth/token; MaxAge=86400", refresh_token)).into(),
        )],
    ).into_response()
}
