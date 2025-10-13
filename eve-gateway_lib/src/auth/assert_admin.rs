use axum::extract::Request;
use axum::middleware::Next;
use axum::response::IntoResponse;
use reqwest::StatusCode;

const HEADER_IS_ADMIN: &str = "X-SF-IsAdmin";

/// Asserts that the requesting character is in the admins list
/// If the character is not in the list, a 403 FORBIDDEN will be returned
/// Otherwise the route will continue
pub async fn assert_admin(
    request:    Request,
    next:       Next,
) -> impl IntoResponse {
    let is_admin = if let Some(x) = request
        .headers()
        .get(HEADER_IS_ADMIN) {
        x.to_str().unwrap_or_default().parse::<bool>().unwrap_or(false)
    } else {
        tracing::error!("could not `assert_admin`, reason: 'no {HEADER_IS_ADMIN} header'");
        return (
            StatusCode::UNAUTHORIZED,
        ).into_response();
    };

    if is_admin {
        next.run(request).await
    } else {
        return (
            StatusCode::FORBIDDEN,
        ).into_response();
    }
}
