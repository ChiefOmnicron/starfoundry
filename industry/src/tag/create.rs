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

use crate::api_docs::{BadRequest, InternalServerError, Unauthorized};
use crate::tag::error::{Result, TagError};
use crate::tag::{TagAuto, TagType};
use crate::{AppState, TagUuid};

/// Create Tag
///
/// - Alternative route: `/latest/tags`
/// - Alternative route: `/v1/tags`
///
/// ---
///
/// Creates a new tag
///
/// ## Security
/// - authenticated
/// - project_group:read
#[utoipa::path(
    post,
    path = "/",
    tag = "tags",
    request_body = CreateTag,
    responses(
        (
            body = CreateTagResponse,
            description = "UUID of the new tag",
            status = OK,
        ),
        (
            description = "There aren't any projects matching the filter",
            status = NO_CONTENT,
        ),
        BadRequest,
        Unauthorized,
        InternalServerError,
    ),
    security(
        ("api_key" = [])
    ),
)]
pub async fn api(
    identity: ExtractIdentity,
    State(state): State<AppState>,
    Json(tag_info): Json<CreateTag>,
) -> Result<impl IntoResponse> {
    let id = create(&state.postgres, identity.character_id, tag_info).await?;

    Ok((
        StatusCode::CREATED,
        Json(CreateTagResponse {
            id,
        }),
    ))
}

pub async fn create(
    pool: &PgPool,
    character_id: CharacterId,
    tag_info: CreateTag,
) -> Result<TagUuid> {
    tag_info.validate()?;

    let mut transaction = pool
        .begin()
        .await
        .map_err(TagError::TransactionError)?;
    let tag_id = sqlx::query!(
        r#"
            INSERT INTO tag
            (
                owner_id,
                content,
                color,
                typ
            )
            VALUES ($1, $2, $3, $4)
            RETURNING id
        "#,
        *character_id,
        tag_info.content,
        tag_info.color,
        &tag_info
            .typ
            .as_str(),
    )
    .fetch_one(&mut *transaction)
    .await
    .map_err(TagError::Create)?;

    if tag_info.typ == TagType::Auto {
        let mut options = Vec::new();
        let mut compares = Vec::new();
        let mut values = Vec::new();

        for auto_tag in tag_info.auto {
            options.push(
                auto_tag
                    .select
                    .as_str()
                    .into(),
            );
            compares.push(
                auto_tag
                    .compare
                    .as_str()
                    .into(),
            );
            values.push(auto_tag.value);
        }

        sqlx::query!(
            "
                INSERT INTO tag_auto
                (
                    tag_id,
                    option,
                    compare,
                    value
                )
                SELECT $1, * FROM UNNEST(
                    $2::VARCHAR[],
                    $3::VARCHAR[],
                    $4::VARCHAR[]
                )
            ",
            tag_id.id,
            &options,
            &compares,
            &values,
        )
        .execute(&mut *transaction)
        .await
        .map_err(TagError::Create)?;
    }

    transaction
        .commit()
        .await
        .map_err(TagError::TransactionError)?;
    Ok(tag_id
        .id
        .into())
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CreateTag {
    pub color:   String,
    pub content: String,
    pub typ:     TagType,

    #[serde(default)]
    pub auto: Vec<TagAuto>,
}

impl CreateTag {
    pub fn validate(&self) -> Result<bool> {
        if self.typ == TagType::Auto
            && self
                .auto
                .is_empty()
        {
            return Err(TagError::ValidationError("Field 'auto' must contain at least one value, when 'typ' is 'AUTO'".into()));
        }

        Ok(true)
    }
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(
    example = json!({
        "id": "fd324c9f-ecda-49c8-948e-18f4b4b23bff"
    })
)]
pub struct CreateTagResponse {
    pub id: TagUuid,
}

impl Default for CreateTagResponse {
    fn default() -> Self {
        Self {
            id: Uuid::default().into(),
        }
    }
}
