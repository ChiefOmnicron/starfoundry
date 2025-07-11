use axum::extract::State;
use axum::response::IntoResponse;
use axum_extra::extract::CookieJar;

use crate::AppStateExtractor;
use axum::http::StatusCode;
use crate::auth::JwtToken;

/// Refresh Token
/// 
/// Obtains a new JWT-Token, if the refresh token is valid
/// 
#[utoipa::path(
    get,
    operation_id = "refresh_token",
    path = "/refresh-token",
    tag = "auth",
    responses(
    ),
)]
pub async fn refresh_token(
    State(state): AppStateExtractor,
    jar: CookieJar,
) -> impl IntoResponse {
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
