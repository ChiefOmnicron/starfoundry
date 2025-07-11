mod member;
mod service;

pub use self::member::*;
pub use self::service::*;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;

use crate::api_docs::{Forbidden, InternalServerError, Unauthorized};
use crate::AppState;
use crate::project_group::error::Result;
use crate::project_group::ProjectGroupUuid;

/// List Group Members
/// 
/// - Alternative route: `/latest/project-groups/{ProjectGroupUuid}/members`
/// - Alternative route: `/v1/project-groups/{ProjectGroupUuid}/members`
/// 
/// Lists all members of a group
/// 
/// ## Security
/// - authenticated
/// - project_group:read
/// 
#[utoipa::path(
    get,
    path = "/{ProjectGroupUuid}/members",
    tag = "project-groups",
    params(
        ProjectGroupUuid,
    ),
    responses(
        (
            body = Vec<ProjectGroupMember>,
            description = "Members of the group",
            status = OK,
        ),
        (
            description = "There aren't any members",
            status = NO_CONTENT,
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
    State(state):             State<AppState>,
    Path(project_group_uuid): Path<ProjectGroupUuid>,
) -> Result<impl IntoResponse> {
    let data = list_members(
            &state.pool,
            project_group_uuid,
        )
        .await?;

    if data.is_empty() {
        Ok(
            (
                StatusCode::NO_CONTENT,
                Json(data),
            )
            .into_response()
        )
    } else {
        Ok(
            (
                StatusCode::OK,
                Json(data),
            )
            .into_response()
        )
    }
}

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::extract::Request;
    use axum::http::header::AUTHORIZATION;
    use axum::http::StatusCode;
    use http_body_util::BodyExt;
    use sqlx::PgPool;
    use starfoundry_libs_types::CharacterId;
    use tower::ServiceExt;

    use crate::auth::JwtToken;
    use crate::test_util::credential_cache;
    use crate::project_group::list_members::ProjectGroupMember;

    #[sqlx::test(
        fixtures("base"),
        migrator = "crate::test_util::MIGRATOR"
    )]
    async fn happy_path(
        pool: PgPool,
    ) {
        let credential_cache = credential_cache(pool.clone()).await;
        let state = crate::AppState {
            pool: pool.clone(),
            credential_cache: credential_cache,
        };
        let (app, _) = crate::project_group::routes(state.clone()).split_for_parts();
        let app = app.with_state(state.clone());

        let token = JwtToken::new(CharacterId(1));
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/00000000-0000-0000-0000-000000000001/members")
                    .method("GET")
                    .header(AUTHORIZATION, token.generate().unwrap())
                    .body(Body::empty())
                    .unwrap()
                )
                .await
                .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body: Vec<ProjectGroupMember> = serde_json::from_slice(
            &response.into_body().collect().await.unwrap().to_bytes()
        ).unwrap();
        assert_eq!(body.len(), 2);
    }

    #[sqlx::test(
        fixtures("base"),
        migrator = "crate::test_util::MIGRATOR"
    )]
    async fn unauthorized(
        pool: PgPool,
    ) {
        let credential_cache = credential_cache(pool.clone()).await;
        let state = crate::AppState {
            pool: pool.clone(),
            credential_cache: credential_cache,
        };
        let (app, _) = crate::project_group::routes(state.clone()).split_for_parts();
        let app = app.with_state(state.clone());

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/00000000-0000-0000-0000-000000000001/members")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap()
                )
                .await
                .unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[sqlx::test(
        fixtures("base"),
        migrator = "crate::test_util::MIGRATOR"
    )]
    async fn forbidden(
        pool: PgPool,
    ) {
        let credential_cache = credential_cache(pool.clone()).await;
        let state = crate::AppState {
            pool: pool.clone(),
            credential_cache: credential_cache,
        };
        let (app, _) = crate::project_group::routes(state.clone()).split_for_parts();
        let app = app.with_state(state.clone());

        let token = JwtToken::new(CharacterId(1));
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/00000000-0000-0000-0000-000000000005/members")
                    .method("GET")
                    .header(AUTHORIZATION, token.generate().unwrap())
                    .body(Body::empty())
                    .unwrap()
                )
                .await
                .unwrap();
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }

    #[sqlx::test(
        fixtures("base"),
        migrator = "crate::test_util::MIGRATOR"
    )]
    async fn not_found(
        pool: PgPool,
    ) {
        let credential_cache = credential_cache(pool.clone()).await;
        let state = crate::AppState {
            pool: pool.clone(),
            credential_cache: credential_cache,
        };
        let (app, _) = crate::project_group::routes(state.clone()).split_for_parts();
        let app = app.with_state(state.clone());

        let token = JwtToken::new(CharacterId(1));
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/00000000-0000-0000-0000-000000000000/members")
                    .method("GET")
                    .header(AUTHORIZATION, token.generate().unwrap())
                    .body(Body::empty())
                    .unwrap()
                )
                .await
                .unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
