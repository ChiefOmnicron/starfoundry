use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use utoipa::ToSchema;

use crate::api_docs::{
    Forbidden, InternalServerError, NotFound, Unauthorized, UnprocessableEntity, UnsupportedMediaType
};
use crate::project_group::ProjectGroupError;
use crate::project_group::error::Result;
use crate::{AppState, ProjectGroupUuid};

/// Update General Group
///
/// - Alternative route: `/v1/project-groups/{ProjectGroupUuid}`
/// - Alternative route: `/latest/project-groups/{ProjectGroupUuid}`
///
/// ---
///
/// Updates a project group
///
/// ## Security
/// - authenticated
/// - project_group:write
#[utoipa::path(
    put,
    path = "/{ProjectGroupUuid}",
    tag = "Project Groups",
    request_body = UpdateProjectGroup,
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
    State(state): State<AppState>,
    Path(project_group_uuid): Path<ProjectGroupUuid>,
    Json(update_info): Json<UpdateProjectGroup>,
) -> Result<impl IntoResponse> {
    update(&state.postgres, project_group_uuid, update_info).await?;

    Ok((StatusCode::NO_CONTENT,))
}

pub async fn update(
    pool: &PgPool,
    project_group_uuid: ProjectGroupUuid,
    update_info: UpdateProjectGroup,
) -> Result<()> {
    update_info.valid()?;

    sqlx::query!(
        "
        UPDATE project_group
        SET
            name = $2,
            description = $3
        WHERE id = $1
    ",
        *project_group_uuid,
        update_info.name,
        update_info.description,
    )
    .execute(pool)
    .await
    .map(drop)
    .map_err(|e| ProjectGroupError::UpdateGroup(e, project_group_uuid))
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct UpdateProjectGroup {
    pub name:        String,
    pub description: Option<String>,
}

impl UpdateProjectGroup {
    pub fn valid(&self) -> Result<bool> {
        if self.name.len() <= 100 {
            if self
                .name
                .trim()
                .is_empty()
            {
                return Err(ProjectGroupError::ValidationError(
                    "Field 'name' must be set".into(),
                ));
            }
        } else {
            return Err(ProjectGroupError::ValidationError(
                "Field 'name' is too long, max length: 100".into(),
            ));
        };

        match &self.description {
            Some(x) => {
                if x.len() >= 10_000 {
                    return Err(ProjectGroupError::ValidationError(
                        "Field 'description' is too long, max length: 10_000"
                            .into(),
                    ));
                }
                Some(x)
            },
            None => None,
        };

        Ok(true)
    }
}

#[cfg(test)]
mod update_project_group_test {
    use std::str::FromStr;

    use axum::body::Body;
    use axum::extract::Request;
    use axum::http::StatusCode;
    use axum::http::header::{CONTENT_TYPE, HOST};
    use sqlx::PgPool;
    use starfoundry_gateway::{
        HEADER_CHARACTER_ID, HEADER_CORPORATION_ID, HEADER_SERVICE
    };
    use uuid::Uuid;

    use crate::project_group::error::ProjectGroupError;
    use crate::project_group::project_group_test_routes;
    use crate::project_group::update::UpdateProjectGroup;

    #[sqlx::test(fixtures(path = "./fixtures", scripts("base")))]
    async fn no_body(pool: PgPool) {
        let result = super::update(
            &pool,
            Uuid::from_str("00000000-0000-0000-0000-000000000001")
                .unwrap()
                .into(),
            UpdateProjectGroup {
                name:        String::new(),
                description: None,
            },
        )
        .await;
        assert!(result.is_err());
        assert!(matches!(result, Err(ProjectGroupError::ValidationError(_))));
    }

    #[sqlx::test(fixtures(path = "./fixtures", scripts("base")))]
    async fn missing_name(pool: PgPool) {
        let result = super::update(
            &pool,
            Uuid::from_str("00000000-0000-0000-0000-000000000001")
                .unwrap()
                .into(),
            UpdateProjectGroup {
                name:        String::new(),
                description: Some(String::from("Test description")),
            },
        )
        .await;
        assert!(result.is_err());
        assert!(matches!(result, Err(ProjectGroupError::ValidationError(_))));
    }

    #[sqlx::test(fixtures(path = "./fixtures", scripts("base")))]
    async fn direct_happy_path(pool: PgPool) {
        let result = super::update(
            &pool,
            Uuid::from_str("00000000-0000-0000-0000-000000000001")
                .unwrap()
                .into(),
            UpdateProjectGroup {
                name:        String::from("My shared projects"),
                description: Some(String::from("My cool description")),
            },
        )
        .await;
        assert!(result.is_ok());

        let entry = sqlx::query!(
                "SELECT * FROM project_group WHERE id = '00000000-0000-0000-0000-000000000001'",
            )
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(entry.name, "My shared projects");
        assert_eq!(
            entry
                .description
                .unwrap(),
            "My cool description"
        );
    }

    #[sqlx::test(fixtures("base"))]
    async fn happy_path(pool: PgPool) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000001")
            .header(CONTENT_TYPE, "application/json")
            .header(HEADER_SERVICE, "industry.test")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .method("PUT")
            .body(Body::new(
                serde_json::to_string(&UpdateProjectGroup {
                    description: Some("Update Description".into()),
                    name:        "Update Name".into(),
                })
                .unwrap(),
            ))
            .unwrap();
        let response = project_group_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::NO_CONTENT);

        let entry = sqlx::query!(
            "
                SELECT pg.*
                FROM project_group pg
                WHERE pg.id = '00000000-0000-0000-0000-000000000001'
            "
        )
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(entry.name, "Update Name");
        assert_eq!(
            entry
                .description
                .unwrap(),
            "Update Description"
        );
    }

    #[sqlx::test(fixtures("base"))]
    async fn unsupported_media_type(pool: PgPool) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000001")
            .header(CONTENT_TYPE, "text/plain")
            .header(HEADER_SERVICE, "industry.test")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .method("PUT")
            .body(Body::new(
                serde_json::to_string(&UpdateProjectGroup {
                    description: Some("Update Description".into()),
                    name:        "Update Name".into(),
                })
                .unwrap(),
            ))
            .unwrap();
        let response = project_group_test_routes(pool, request).await;
        assert_eq!(response.status(), StatusCode::UNSUPPORTED_MEDIA_TYPE);
    }

    #[sqlx::test(fixtures("base"))]
    async fn bad_request_no_body(pool: PgPool) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000001")
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

    #[sqlx::test(fixtures("base"))]
    async fn bad_request_no_name(pool: PgPool) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000001")
            .header(CONTENT_TYPE, "application/json")
            .header(HEADER_SERVICE, "industry.test")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .method("PUT")
            .body(Body::new(
                serde_json::to_string(&serde_json::json!({
                    "description": "My cool description",
                }))
                .unwrap(),
            ))
            .unwrap();
        let response = project_group_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[sqlx::test(fixtures("base"))]
    async fn unauthorized(pool: PgPool) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000001")
            .header(CONTENT_TYPE, "application/json")
            .method("PUT")
            .body(Body::new(
                serde_json::to_string(&UpdateProjectGroup {
                    description: Some("Update Description".into()),
                    name:        "Update Name".into(),
                })
                .unwrap(),
            ))
            .unwrap();
        let response = project_group_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[sqlx::test(fixtures("base"))]
    async fn forbidden(pool: PgPool) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000001")
            .header(CONTENT_TYPE, "application/json")
            .header(HEADER_SERVICE, "industry.test")
            .header(HEADER_CHARACTER_ID, 2)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .method("PUT")
            .body(Body::new(
                serde_json::to_string(&UpdateProjectGroup {
                    description: Some("Update Description".into()),
                    name:        "Update Name".into(),
                })
                .unwrap(),
            ))
            .unwrap();
        let response = project_group_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }

    #[sqlx::test(fixtures("base"))]
    async fn not_found(pool: PgPool) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000010")
            .header(CONTENT_TYPE, "application/json")
            .header(HEADER_SERVICE, "industry.test")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .method("PUT")
            .body(Body::new(
                serde_json::to_string(&UpdateProjectGroup {
                    description: Some("Update Description".into()),
                    name:        "Update Name".into(),
                })
                .unwrap(),
            ))
            .unwrap();
        let response = project_group_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
