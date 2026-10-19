use std::str::FromStr;

use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use starfoundry_gateway::ExtractIdentity;
use starfoundry_lib_types::CharacterId;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::api_docs::{
    BadRequest, InternalServerError, Unauthorized, UnprocessableEntity, UnsupportedMediaType
};
use crate::project_group::ProjectGroupError;
use crate::project_group::error::Result;
use crate::project_group::permission::ProjectGroupPermissionCode;
use crate::{AppState, ProjectGroupUuid};

/// Create Group
///
/// - Alternative route: `/latest/project-groups`
/// - Alternative route: `/v1/project-groups`
///
/// ---
///
/// Creates a new project group
///
/// ## Security
/// - authenticated
#[utoipa::path(
    post,
    path = "/",
    tag = "Project Groups",
    request_body = CreateProjectGroup,
    responses(
        (
            body = CreateProjectGroupResponse,
            description = "Id of the new project group",
            status = CREATED,
        ),
        BadRequest,
        Unauthorized,
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
    identity: ExtractIdentity,
    Json(info): Json<CreateProjectGroup>,
) -> Result<impl IntoResponse> {
    // TODO: add initial time_split table
    let id = create(&state.postgres, identity.character_id, info).await?;

    Ok((
        StatusCode::CREATED,
        Json(CreateProjectGroupResponse {
            id,
        }),
    ))
}

pub async fn create(
    pool: &PgPool,
    character_id: CharacterId,
    info: CreateProjectGroup,
) -> Result<ProjectGroupUuid> {
    info.validate()?;

    let mut transaction = pool
        .begin()
        .await
        .map_err(ProjectGroupError::TransactionError)?;

    // create the group
    let group_id = sqlx::query!(
        "
            INSERT INTO project_group(
                owner,
                name
            )
            VALUES ($1, $2)
            RETURNING id
        ",
        *character_id,
        info.name,
    )
    .fetch_one(&mut *transaction)
    .await
    .map(|x| ProjectGroupUuid::new(x.id))
    .map_err(ProjectGroupError::CreateGroup)?;

    // add the owner as member of the group
    sqlx::query!(
        "
            INSERT INTO project_group_member(
                project_group_id,
                character_id,
                permission
            )
            VALUES ($1, $2, $3)
        ",
        *group_id,
        *character_id,
        *ProjectGroupPermissionCode::Owner,
    )
    .execute(&mut *transaction)
    .await
    .map_err(ProjectGroupError::CreateGroup)?;

    // add the defaults
    sqlx::query!(
        "
            INSERT INTO project_group_default_market(
                project_group_id,
                structure_id
            )
            VALUES (
                $1, $2
            )
        ",
        *group_id,
        Uuid::from_str("00000000-0000-0000-0000-000000000001")
            .unwrap_or_default(), // jita
    )
    .execute(&mut *transaction)
    .await
    .map_err(ProjectGroupError::CreateGroup)?;

    transaction
        .commit()
        .await
        .map_err(ProjectGroupError::TransactionError)?;
    Ok(group_id)
}

#[derive(Debug, Serialize, ToSchema)]
#[schema(
    example = json!({
        "id": "fd324c9f-ecda-49c8-948e-18f4b4b23bff"
    })
)]
pub struct CreateProjectGroupResponse {
    id: ProjectGroupUuid,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(
    example = json!({
        "name": "My cool group"
    })
)]
pub struct CreateProjectGroup {
    /// Maximum length 100
    pub name: String,
}

impl CreateProjectGroup {
    pub fn validate(&self) -> Result<bool> {
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

        Ok(true)
    }
}

#[cfg(test)]
mod create_project_group_test {
    use axum::body::Body;
    use axum::extract::Request;
    use axum::http::StatusCode;
    use axum::http::header::{CONTENT_TYPE, HOST};
    use http_body_util::BodyExt;
    use serde::Deserialize;
    use sqlx::PgPool;
    use starfoundry_gateway::{
        HEADER_CHARACTER_ID, HEADER_CORPORATION_ID, HEADER_SERVICE
    };
    use starfoundry_lib_types::CharacterId;
    use uuid::Uuid;

    use crate::project_group::create::CreateProjectGroup;
    use crate::project_group::error::ProjectGroupError;
    use crate::project_group::project_group_test_routes;

    #[sqlx::test()]
    async fn no_body(pool: PgPool) {
        let result = super::create(
            &pool,
            CharacterId(1),
            CreateProjectGroup {
                name: String::new(),
            },
        )
        .await;
        assert!(result.is_err());
        assert!(matches!(result, Err(ProjectGroupError::ValidationError(_))));
    }

    #[sqlx::test()]
    async fn empty_name(pool: PgPool) {
        let result = super::create(
            &pool,
            CharacterId(1),
            CreateProjectGroup {
                name: String::new(),
            },
        )
        .await;
        assert!(result.is_err());
        assert!(matches!(result, Err(ProjectGroupError::ValidationError(_))));
    }

    #[sqlx::test()]
    async fn direct_happy_path(pool: PgPool) {
        let result = super::create(
            &pool,
            CharacterId(1),
            CreateProjectGroup {
                name: String::from("My shared projects"),
            },
        )
        .await;
        assert!(result.is_ok());

        let entry = sqlx::query!(
            "SELECT * FROM project_group WHERE id = $1",
            *result.unwrap(),
        )
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(entry.name, "My shared projects");
    }

    #[sqlx::test(fixtures("base"))]
    async fn happy_path(pool: PgPool) {
        #[derive(Deserialize)]
        struct ResponseId {
            id: Uuid,
        }

        let request = Request::builder()
            .uri("/")
            .header(CONTENT_TYPE, "application/json")
            .header(HEADER_SERVICE, "industry.test")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .method("POST")
            .body(Body::new(
                serde_json::to_string(&CreateProjectGroup {
                    name: "My shared projects".into(),
                })
                .unwrap(),
            ))
            .unwrap();
        let response = project_group_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::CREATED);
        let body: ResponseId = serde_json::from_slice(
            &response
                .into_body()
                .collect()
                .await
                .unwrap()
                .to_bytes(),
        )
        .unwrap();

        let entry = sqlx::query!("
                SELECT pg.*
                FROM project_group pg
                JOIN project_group_member pgm ON pgm.project_group_id = pg.id
                JOIN project_group_default_market pgdm ON pgdm.project_group_id = pg.id
                WHERE pg.id = $1
            ",
                body.id,
            )
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(entry.name, "My shared projects");
    }

    #[sqlx::test(fixtures("base"))]
    async fn unsupported_media_type(pool: PgPool) {
        let request = Request::builder()
            .uri("/")
            .header(CONTENT_TYPE, "text/plain")
            .header(HEADER_SERVICE, "industry.test")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .method("POST")
            .body(Body::new(
                serde_json::to_string(&CreateProjectGroup {
                    name: "My shared projects".into(),
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
            .uri("/")
            .header(CONTENT_TYPE, "application/json")
            .header(HEADER_SERVICE, "industry.test")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .method("POST")
            .body(Body::empty())
            .unwrap();
        let response = project_group_test_routes(pool.clone(), request).await;
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
