use axum_extra::extract::CookieJar;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;

use crate::AppState;
use crate::auth::JwtToken;
use crate::api_docs::Unauthorized;

/// Refresh Token
/// 
/// Alternative route: `/latest/auth/refresh-token`
/// Alternative route: `/v1/auth/refresh-token`
/// 
/// ---
/// 
/// Obtains a new JWT-Token, if the refresh token is valid
/// 
#[utoipa::path(
    get,
    operation_id = "refresh_token",
    path = "/refresh-token",
    tag = "auth",
    responses(
        (
            status = OK,
            description = "JWT-Token in the body",
            body = String,
            content_type = "application/json",
        ),
        Unauthorized,
    ),
)]
pub async fn refresh_token(
    State(state): State<AppState>,
    jar: CookieJar,
) -> impl IntoResponse {
    // TODO: add generated jwt token to a cache
    if let Some(x) = jar.get("refresh_token").map(|x| x.value()) {
        let character_id = sqlx::query!("
                SELECT character_id
                FROM jwt_refresh_token
                WHERE refresh_token = $1
            ",
                x
            )
            .fetch_optional(&state.pool)
            .await
            .unwrap();

        if let Some(record) = character_id {
            let character_id = record.character_id;

            (
                StatusCode::OK,
                JwtToken::new(character_id.into()).generate().unwrap(),
            ).into_response()
        } else {
            (
                StatusCode::UNAUTHORIZED
            ).into_response()
        }
    } else {
        (
            StatusCode::UNAUTHORIZED
        ).into_response()
    }
}
