use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use sqlx::PgPool;
use starfoundry_gateway::ExtractIdentity;
use starfoundry_lib_types::CharacterId;

use crate::api_docs::{Forbidden, InternalServerError, NotFound, Unauthorized};
use crate::structure::StructureError;
use crate::structure::error::Result;
use crate::{AppState, StructureUuid};

/// Delete Structure
///
/// - Alternative route: `/latest/structures/{StructureUuid}`
/// - Alternative route: `/v1/structures/{StructureUuid}`
///
/// ---
///
/// Deletes a structure
///
/// ## Security
/// - authenticated
/// - structure:write
#[utoipa::path(
    delete,
    path = "/{StructureUuid}",
    tag = "Structures",
    params(
        StructureUuid,
    ),
    responses(
        (
            description = "The structure was successfully deleted",
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
    Path(structure_uuid): Path<StructureUuid>,
) -> Result<impl IntoResponse> {
    delete(&state.postgres, identity.character_id, structure_uuid).await?;

    Ok((StatusCode::NO_CONTENT, ()).into_response())
}

pub async fn delete(
    pool: &PgPool,
    character_id: CharacterId,
    structure_id: StructureUuid,
) -> Result<StructureUuid> {
    sqlx::query!(
        "
        DELETE FROM structure
        WHERE owner = $1
            AND id = $2
        RETURNING id
    ",
        *character_id,
        *structure_id,
    )
    .fetch_one(pool)
    .await
    .map(|x| StructureUuid::new(x.id))
    .map_err(|e| StructureError::DeleteStructure(e, structure_id))
}

#[cfg(test)]
mod delete_project_group_test {
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

    use crate::structure::structure_test_routes;

    #[sqlx::test(fixtures(path = "./fixtures", scripts("base")))]
    async fn direct_happy_path(pool: PgPool) {
        let result = super::delete(
            &pool,
            CharacterId(1),
            Uuid::from_str("00000000-0000-0000-0000-000000000003")
                .unwrap()
                .into(),
        )
        .await;
        assert!(result.is_ok());
    }

    #[sqlx::test(fixtures("base"))]
    async fn happy_path(pool: PgPool) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000001")
            .method("DELETE")
            .header(HEADER_SERVICE, "industry.test")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .body(Body::empty())
            .unwrap();
        let response = structure_test_routes(pool, request).await;
        assert_eq!(response.status(), StatusCode::NO_CONTENT);
    }

    #[sqlx::test(fixtures("base"))]
    async fn unauthorized(pool: PgPool) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000001")
            .method("DELETE")
            .body(Body::empty())
            .unwrap();
        let response = structure_test_routes(pool, request).await;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[sqlx::test(fixtures("base"))]
    async fn forbidden(pool: PgPool) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000002")
            .method("DELETE")
            .header(HEADER_SERVICE, "industry.test")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .body(Body::empty())
            .unwrap();
        let response = structure_test_routes(pool, request).await;
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }

    #[sqlx::test(fixtures("base"))]
    async fn not_found(pool: PgPool) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000000")
            .method("DELETE")
            .header(HEADER_SERVICE, "industry.test")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .body(Body::empty())
            .unwrap();
        let response = structure_test_routes(pool, request).await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
