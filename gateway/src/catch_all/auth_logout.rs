use axum::response::IntoResponse;
use reqwest::header::{HOST, LOCATION, SET_COOKIE};
use reqwest::StatusCode;
use axum::http::HeaderMap;

pub async fn catch_all_auth_logout(
    headers: HeaderMap,
) -> impl IntoResponse {
    let host = if let Some(x) = headers.get(HOST) {
        x.to_str().unwrap_or_default()
    } else {
        return (
            StatusCode::BAD_REQUEST,
        ).into_response()
    };

    (
        StatusCode::FOUND,
        [(
            LOCATION,
            format!("https://{host}"),
        ), (
            SET_COOKIE,
            format!("refresh_token=deleted; HttpOnly; Secure; SameSite=Strict; Path=/api/auth/token; Expires=Thu, 01 Jan 1970 00:00:00 GMT"),
        )],
    ).into_response()
}
