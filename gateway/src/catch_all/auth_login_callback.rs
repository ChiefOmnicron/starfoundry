use axum::extract::Query;
use axum::http::{HeaderMap, StatusCode};
use axum::http::header::{LOCATION, SET_COOKIE};
use axum::response::IntoResponse;
use reqwest::header::HOST;
use std::collections::HashMap;

pub async fn catch_all_auth_login_callback(
    Query(params): Query<HashMap<String, String>>,
    headers:       HeaderMap,
) -> impl IntoResponse {
    let host = if let Some(x) = headers.get(HOST) {
        x.to_str().unwrap_or_default()
    } else {
        return (
            StatusCode::BAD_REQUEST,
        ).into_response()
    };

    let refresh_token = if let Some(x) = params.get("refresh_token") {
        x.clone()
    } else {
        return (
            StatusCode::FOUND,
            [(
                LOCATION,
                format!("https://{host}/auth/forbidden"),
            )]
        ).into_response()
    };

    (
        StatusCode::FOUND,
        [(
            LOCATION,
            format!("https://{host}"),
        ), (
            SET_COOKIE,
            format!("refresh_token={refresh_token}; HttpOnly; Secure; SameSite=Strict; Path=/api/auth/token; MaxAge=86400"),
        )],
    ).into_response()
}
