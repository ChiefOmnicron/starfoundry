use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use starfoundry_lib_gateway::ExtractIdentity;

use crate::api_docs::{Forbidden, InternalServerError, Unauthorized};
use crate::AppState;
use crate::industry_hub::error::Result;
use crate::industry_hub::service::{UpdateIndustryHub, update};
use crate::industry_hub::IndustryHubUuid;

/// Update Industry Hub
/// 
/// - Alternative route: `/latest/industry-hubs/{IndustryHubUuid}`
/// - Alternative route: `/v1/industry-hubs/{IndustryHubUuid}`
/// 
/// ---
/// 
/// Lists all available industry hubs
/// 
/// ## Security
/// - authenticated
/// 
#[utoipa::path(
    put,
    path = "/{IndustryHubUuid}",
    tag = "Industry Hubs",
    request_body = UpdateIndustryHub,
    params(
        IndustryHubUuid,
    ),
    responses(
        (
            description = "The structure group was successfully updated",
            status = NO_CONTENT
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
    identity:                ExtractIdentity,
    State(state):            State<AppState>,
    Path(industry_hub_uuid): Path<IndustryHubUuid>,
    Json(industry_hub):      Json<UpdateIndustryHub>,
) -> Result<impl IntoResponse> {
    let data = update(
            &state.pool,
            identity.character_id,
            industry_hub_uuid,
            industry_hub,
        )
        .await?;

    Ok(
        (
            StatusCode::NO_CONTENT,
            Json(data),
        )
        .into_response()
    )
}

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::extract::Request;
    use axum::http::header::{CONTENT_TYPE, HOST};
    use axum::http::StatusCode;
    use sqlx::PgPool;
    use starfoundry_lib_gateway::{HEADER_CHARACTER_ID, HEADER_CORPORATION_ID, HEADER_SERVICE};

    use crate::industry_hub::service::UpdateIndustryHub;
    use crate::industry_hub::industry_hub_test_routes;

    #[sqlx::test(
        fixtures("base"),
    )]
    async fn happy_path_all(
        pool: PgPool,
    ) {
        let request = Request::builder()
            .header(CONTENT_TYPE, "application/json")
            .uri("/00000000-0000-0000-0000-100000000001")
            .method("PUT")
            .header(HEADER_SERVICE, "industry.test")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .body(Body::new(
                serde_json::to_string(&UpdateIndustryHub {
                    name: "My cool structure group".into(),
                    structures: Vec::new(),
                    shares: Vec::new(),
                    description: None,
                }).unwrap()
            ))
            .unwrap();
        let response = industry_hub_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::NO_CONTENT);
    }

    #[sqlx::test(
        fixtures("base"),
    )]
    async fn unauthorized(
        pool: PgPool,
    ) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-100000000001")
            .method("PUT")
            .body(Body::empty())
            .unwrap();
        let response = industry_hub_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[sqlx::test(
        fixtures("base"),
    )]
    async fn forbidden(
        pool: PgPool,
    ) {
        let request = Request::builder()
            .header(CONTENT_TYPE, "application/json")
            .uri("/00000000-0000-0000-0000-100000000002")
            .method("PUT")
            .header(HEADER_SERVICE, "industry.test")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .body(Body::new(
                serde_json::to_string(&UpdateIndustryHub {
                    name: "My cool structure group".into(),
                    structures: Vec::new(),
                    shares: Vec::new(),
                    description: None,
                }).unwrap()
            ))
            .unwrap();
        let response = industry_hub_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }
}
