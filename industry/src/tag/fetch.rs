use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use starfoundry_gateway::ExtractIdentity;
use starfoundry_lib_types::CharacterId;
use utoipa::ToSchema;

use crate::api_docs::{Forbidden, InternalServerError, NotFound, Unauthorized};
use crate::tag::error::{Result, TagError};
use crate::tag::{TagAuto, TagAutoCompare, TagAutoSelect, TagType};
use crate::{AppState, TagUuid};

/// Fetch Tag
///
/// - Alternative route: `/latest/tags/{TagUuid}`
/// - Alternative route: `/v1/tags/{TagUuid}`
///
/// ---
///
/// Fetches information about a tag
///
/// ## Security
/// - authenticated
/// - structure:read
#[utoipa::path(
    get,
    path = "/{TagUuid}",
    tag = "Tags",
    params(
        TagUuid,
    ),
    responses(
        (
            body = Tag,
            description = "Information about the structure",
            status = OK,
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
    Path(tag_id): Path<TagUuid>,
) -> Result<impl IntoResponse> {
    let entry = fetch(&state.postgres, identity.character_id, tag_id).await?;

    if let Some(x) = entry {
        Ok((StatusCode::OK, Json(x)).into_response())
    } else {
        Ok((StatusCode::NO_CONTENT, Json(())).into_response())
    }
}

pub async fn fetch(
    pool: &PgPool,
    character_id: CharacterId,
    tag_id: TagUuid,
) -> Result<Option<Tag>> {
    let entry = sqlx::query!(
        r#"
            SELECT
                id,
                content,
                color,
                typ
            FROM tag
            WHERE
                owner_id = $1 AND
                id = $2
        "#,
        *character_id,
        *tag_id,
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| TagError::Fetch(e, tag_id))?;

    if let Some(x) = entry {
        let typ = TagType::try_from(x.typ)?;

        let triggers = if typ == TagType::Auto {
            let triggers = sqlx::query!(
                "
                    SELECT
                        option,
                        compare,
                        value
                    FROM tag_auto
                    WHERE tag_id = $1
                ",
                *tag_id,
            )
            .fetch_all(pool)
            .await
            .map_err(|e| TagError::Fetch(e, tag_id))?;

            let mut trigger_result = Vec::new();
            for trigger in triggers {
                let select = match TagAutoSelect::try_from(trigger.option) {
                    Ok(x) => x,
                    Err(e) => {
                        tracing::warn!("{}", e.to_string());
                        continue
                    },
                };
                let compare = match TagAutoCompare::try_from(trigger.compare) {
                    Ok(x) => x,
                    Err(e) => {
                        tracing::warn!("{}", e.to_string());
                        continue
                    },
                };
                let value = trigger.value;

                trigger_result.push(TagAuto {
                    compare,
                    select,
                    value,
                });
            }
            trigger_result
        } else {
            Vec::new()
        };

        let tag = Tag {
            id:      x.id.into(),
            color:   x.color,
            content: x.content,
            typ:     typ,
            auto:    triggers,
        };
        Ok(Some(tag))
    } else {
        Ok(None)
    }
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct Tag {
    pub id:      TagUuid,
    pub color:   String,
    pub content: String,
    pub typ:     TagType,

    pub auto: Vec<TagAuto>,
}
