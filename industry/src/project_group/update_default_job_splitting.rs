use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;

use crate::api_docs::{Forbidden, InternalServerError, NotFound, Unauthorized, UnprocessableEntity, UnsupportedMediaType};
use crate::AppState;
use crate::project_group::error::Result;
use crate::project_group::ProjectGroupUuid;
use crate::project_group::service::{UpdateProjectGroupDefaultJobSplitting, update_default_job_splitting};

/// Update Default Job Splitting
/// 
/// - Alternative route: `/v1/project-groups/{ProjectGroupUuid}/defaults/job-splitting`
/// - Alternative route: `/latest/project-groups/{ProjectGroupUuid}/defaults/job-splitting`
/// 
/// ---
/// 
/// Updates a default job splitting
/// 
/// ## Security
/// - authenticated
/// - project_group:write
/// 
#[utoipa::path(
    put,
    path = "/{ProjectGroupUuid}/defaults/job-splitting",
    tag = "Project Groups",
    request_body = Vec<UpdateProjectGroupDefaultJobSplitting>,
    params(
        ProjectGroupUuid,
    ),
    responses(
        (
            description = "The group was updated",
            status = NO_CONTENT,
        ),
        Unauthorized,
        Forbidden,
        NotFound,
        UnsupportedMediaType,
        UnprocessableEntity,
        InternalServerError,
    ),
    security(
        ("api_key" = [])
    ),
)]
pub async fn api(
    State(state):             State<AppState>,
    Path(project_group_uuid): Path<ProjectGroupUuid>,
    Json(update_info):        Json<Vec<UpdateProjectGroupDefaultJobSplitting>>,
) -> Result<impl IntoResponse> {
    update_default_job_splitting(
        &state.pool,
        project_group_uuid,
        update_info,
    ).await?;

    Ok((
        StatusCode::NO_CONTENT,
    ))
}

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::extract::Request;
    use axum::http::header::{CONTENT_TYPE, HOST};
    use axum::http::StatusCode;
    use sqlx::PgPool;
    use starfoundry_lib_gateway::{HEADER_CHARACTER_ID, HEADER_CORPORATION_ID, HEADER_SERVICE};
    use starfoundry_lib_types::TypeId;

    use crate::project_group::project_group_test_routes;
    use crate::project_group::service::UpdateProjectGroupDefaultJobSplitting;

    #[sqlx::test(
        fixtures("base"),
    )]
    async fn happy_path(
        pool: PgPool,
    ) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000001/defaults/job-splitting")
            .header(CONTENT_TYPE, "application/json")
            .header(HEADER_SERVICE, "industry.test")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .method("PUT")
            .body(Body::new(
                serde_json::to_string(&vec![
                    UpdateProjectGroupDefaultJobSplitting {
                        max_runs: 40,
                        type_id:  TypeId(21027),
                    }
                ]).unwrap()
            ))
            .unwrap();
        let response = project_group_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::NO_CONTENT);

        let entry = sqlx::query!("
                SELECT pg.*
                FROM project_group_default_job_splitting_run pg
                WHERE pg.project_group_id = '00000000-0000-0000-0000-000000000001'
            ")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(entry.max_runs, 40);
    }

    #[sqlx::test(
        fixtures("base"),
    )]
    async fn unsupported_media_type(
        pool: PgPool,
    ) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000001/defaults/job-splitting")
            .header(CONTENT_TYPE, "text/plain")
            .header(HEADER_SERVICE, "industry.test")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .method("PUT")
            .body(Body::new(
                serde_json::to_string(&vec![TypeId(1)]).unwrap()
            ))
            .unwrap();
        let response = project_group_test_routes(pool, request).await;
        assert_eq!(response.status(), StatusCode::UNSUPPORTED_MEDIA_TYPE);
    }

    #[sqlx::test(
        fixtures("base"),
    )]
    async fn bad_request_no_body(
        pool: PgPool,
    ) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000001/defaults/job-splitting")
            .header(CONTENT_TYPE, "application/json")
            .header(HEADER_SERVICE, "industry.test")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .method("PUT")
            .body(Body::empty())
            .unwrap();
        let response = project_group_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[sqlx::test(
        fixtures("base"),
    )]
    async fn unauthorized(
        pool: PgPool,
    ) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000001/defaults/job-splitting")
            .header(CONTENT_TYPE, "application/json")
            .method("PUT")
            .body(Body::new(
                serde_json::to_string(&vec![TypeId(1)]).unwrap()
            ))
            .unwrap();
        let response = project_group_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[sqlx::test(
        fixtures("base"),
    )]
    async fn forbidden(
        pool: PgPool,
    ) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000001/defaults/job-splitting")
            .header(CONTENT_TYPE, "application/json")
            .header(HEADER_SERVICE, "industry.test")
            .header(HEADER_CHARACTER_ID, 2)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .method("PUT")
            .body(Body::new(
                serde_json::to_string(&vec![TypeId(1)]).unwrap()
            ))
            .unwrap();
        let response = project_group_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }

    #[sqlx::test(
        fixtures("base"),
    )]
    async fn not_found(
        pool: PgPool,
    ) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000010/defaults/job-splitting")
            .header(CONTENT_TYPE, "application/json")
            .header(HEADER_SERVICE, "industry.test")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .method("PUT")
            .body(Body::new(
                serde_json::to_string(&vec![TypeId(1)]).unwrap()
            ))
            .unwrap();
        let response = project_group_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
