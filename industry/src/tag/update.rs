use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use starfoundry_gateway::ExtractIdentity;
use starfoundry_lib_types::CharacterId;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::api_docs::{
    Forbidden, InternalServerError, NotFound, Unauthorized, UnprocessableEntity, UnsupportedMediaType
};
use crate::tag::error::{Result, TagError};
use crate::tag::{TagAuto, TagType};
use crate::{AppState, ProjectUuid, TagUuid};

/// Update Tag
///
/// - Alternative route: `/v1/tags/{TagUuid}`
/// - Alternative route: `/latest/tags/{TagUuid}`
///
/// ---
///
/// Updates the tag
///
/// ## Security
/// - authenticated
/// - project_group:write
#[utoipa::path(
    put,
    path = "/{TagUuid}",
    tag = "Tags",
    request_body = UpdateTag,
    params(
        TagUuid,
    ),
    responses(
        (
            description = "The tag was updated",
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
    identity: ExtractIdentity,
    State(state): State<AppState>,
    Path(tag_id): Path<TagUuid>,
    Json(update_info): Json<UpdateTag>,
) -> Result<impl IntoResponse> {
    update(&state.postgres, identity.character_id, tag_id, update_info).await?;

    Ok((StatusCode::NO_CONTENT,))
}

pub async fn update(
    pool: &PgPool,
    character_id: CharacterId,
    tag_id: TagUuid,
    tag_info: UpdateTag,
) -> Result<()> {
    tag_info.validate()?;

    let mut transaction = pool
        .begin()
        .await
        .map_err(TagError::TransactionError)?;
    sqlx::query!(
        r#"
            DELETE FROM tag_auto
            WHERE tag_id = $1
        "#,
        *tag_id,
    )
    .execute(&mut *transaction)
    .await
    .map(drop)
    .map_err(TagError::Update)?;

    sqlx::query!(
        "
            UPDATE tag
            SET
                content = $3,
                color = $4,
                typ = $5
            WHERE
                owner_id = $1 AND
                id = $2
        ",
        *character_id,
        *tag_id,
        tag_info.content,
        tag_info.color,
        tag_info
            .typ
            .as_str(),
    )
    .execute(&mut *transaction)
    .await
    .map_err(TagError::Update)?;

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
            *tag_id,
            &options,
            &compares,
            &values,
        )
        .execute(&mut *transaction)
        .await
        .map_err(TagError::Update)?;
    }

    transaction
        .commit()
        .await
        .map_err(TagError::TransactionError)?;
    Ok(())
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct UpdateTag {
    pub color:   String,
    pub content: String,
    pub typ:     TagType,

    pub auto: Vec<TagAuto>,
}

impl UpdateTag {
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
pub struct UpdateTagResponse {
    pub id: ProjectUuid,
}

impl Default for UpdateTagResponse {
    fn default() -> Self {
        Self {
            id: Uuid::default().into(),
        }
    }
}
