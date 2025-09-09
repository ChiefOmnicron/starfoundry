use axum::extract::Request;
use axum::middleware::Next;
use axum::response::IntoResponse;
use reqwest::header::AUTHORIZATION;
use reqwest::StatusCode;

/// Only checks if the authorization header exists
pub async fn assert_login(
    request: Request,
    next: Next,
) -> impl IntoResponse {
    if let None = request
        .headers()
        .get(AUTHORIZATION) {

        return (
            StatusCode::UNAUTHORIZED,
        ).into_response();
    }

    next.run(request).await
}
