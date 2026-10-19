use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use sqlx::PgPool;
use starfoundry_gateway::ExtractIdentity;
use starfoundry_lib_types::CharacterId;

use crate::api_docs::{Forbidden, InternalServerError, NotFound, Unauthorized};
use crate::industry_hub::IndustryHubError;
use crate::industry_hub::error::Result;
use crate::{AppState, IndustryHubUuid};

/// Delete Industry Hub
///
/// - Alternative route: `/latest/industry-hubs/{IndustryHubUuid}`
/// - Alternative route: `/v1/industry-hubs/{IndustryHubUuid}`
///
/// ---
///
/// Deletes an industry hub
///
/// ## Security
/// - authenticated
#[utoipa::path(
    delete,
    path = "/{IndustryHubUuid}",
    tag = "Industry Hubs",
    params(
        IndustryHubUuid,
    ),
    responses(
        (
            description = "The industry hub was successfully deleted",
            status = NO_CONTENT,
        ),
        Unauthorized,
        Forbidden,
        NotFound,
        InternalServerError,
    ),
    security(
        ("api_key" = [])
    ),
)]
pub async fn api(
    identity: ExtractIdentity,
    State(state): State<AppState>,
    Path(industry_hub_uuid): Path<IndustryHubUuid>,
) -> Result<impl IntoResponse> {
    delete(&state.postgres, identity.character_id, industry_hub_uuid).await?;

    Ok((StatusCode::NO_CONTENT, ()).into_response())
}

pub async fn delete(
    pool: &PgPool,
    character_id: CharacterId,
    industry_hub_id: IndustryHubUuid,
) -> Result<IndustryHubUuid> {
    sqlx::query!(
        "
        DELETE FROM industry_hub
        WHERE owner = $1
            AND id = $2
        RETURNING id
    ",
        *character_id,
        *industry_hub_id,
    )
    .fetch_one(pool)
    .await
    .map(|x| IndustryHubUuid::new(x.id))
    .map_err(|e| IndustryHubError::DeleteIndustryHub(e, industry_hub_id))
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use axum::body::Body;
    use axum::extract::Request;
    use axum::http::StatusCode;
    use axum::http::header::HOST;
    use sqlx::PgPool;
    use starfoundry_gateway::{
        HEADER_CHARACTER_ID, HEADER_CORPORATION_ID, HEADER_SERVICE
    };
    use starfoundry_lib_types::CharacterId;
    use uuid::Uuid;

    use crate::industry_hub::industry_hub_test_routes;

    #[sqlx::test(fixtures(path = "./fixtures", scripts("base")))]
    async fn direct_happy_path(pool: PgPool) {
        let result = super::delete(
            &pool,
            CharacterId(1),
            Uuid::from_str("00000000-0000-0000-0000-100000000001")
                .unwrap()
                .into(),
        )
        .await;
        assert!(result.is_ok());
    }

    #[sqlx::test(fixtures("base"))]
    async fn happy_path(pool: PgPool) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-100000000001")
            .method("GET")
            .header(HEADER_SERVICE, "industry.test")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .body(Body::empty())
            .unwrap();
        let response = industry_hub_test_routes(pool, request).await;
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[sqlx::test(fixtures("base"))]
    async fn unauthorized(pool: PgPool) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-100000000001")
            .method("GET")
            .header(HEADER_SERVICE, "industry.test")
            .header(HOST, "test.starfoundry.space")
            .body(Body::empty())
            .unwrap();
        let response = industry_hub_test_routes(pool, request).await;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[sqlx::test(fixtures("base"))]
    async fn forbidden(pool: PgPool) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-100000000002")
            .method("GET")
            .header(HEADER_SERVICE, "industry.test")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .body(Body::empty())
            .unwrap();
        let response = industry_hub_test_routes(pool, request).await;
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }

    #[sqlx::test(fixtures("base"))]
    async fn not_found(pool: PgPool) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000000")
            .method("GET")
            .header(HEADER_SERVICE, "industry.test")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .body(Body::empty())
            .unwrap();
        let response = industry_hub_test_routes(pool, request).await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
