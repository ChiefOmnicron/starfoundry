use axum::extract::Request;
use axum::middleware::Next;
use axum::response::IntoResponse;
use reqwest::StatusCode;

use crate::ExtractIdentity;

/// Asserts that the requesting character is in the admins list
/// If the character is not in the list, a 403 FORBIDDEN will be returned
/// Otherwise the route will continue
pub async fn assert_admin(
    identity:   ExtractIdentity,
    request:    Request,
    next:       Next,
) -> impl IntoResponse {
    if identity.is_admin {
        next.run(request).await
    } else {
        return (
            StatusCode::FORBIDDEN,
        ).into_response();
    }
}
