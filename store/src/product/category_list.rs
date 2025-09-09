use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;

use crate::api_docs::{Forbidden, InternalServerError, Unauthorized};
use crate::AppState;
use crate::product::error::{ProductError, Result};

/// List Categories
/// 
/// - Alternative route: `/latest/categories`
/// - Alternative route: `/v1/categories`
/// 
/// ---
/// 
/// Lists all available categories
/// 
/// ## Security
/// - authenticated
/// - structure:read
/// 
#[utoipa::path(
    get,
    path = "/categories",
    tag = "categories",
    responses(
        (
            body = Vec<String>,
            description = "All categories",
            status = OK,
        ),
        Unauthorized,
        Forbidden,
        InternalServerError,
    ),
    security(
        ("api_key" = [])
    ),
)]
pub async fn api(
    State(state): State<AppState>,
) -> Result<impl IntoResponse> {
    let categories = sqlx::query!("
            SELECT DISTINCT category
            FROM product
        ")
        .fetch_all(&state.postgres)
        .await
        .map_err(ProductError::GeneralSqlxError)?;
    let categories = categories
        .into_iter()
        .map(|x| x.category)
        .collect::<Vec<_>>();

    Ok(
        (
            StatusCode::OK,
            Json(categories),
        )
        .into_response()
    )
}
