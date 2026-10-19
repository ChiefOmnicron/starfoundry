use axum::Json;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use starfoundry_gateway::ExtractIdentity;
use starfoundry_lib_types::CharacterId;
use utoipa::{IntoParams, ToSchema};

use crate::AppState;
use crate::api_docs::{BadRequest, InternalServerError, Unauthorized};
use crate::tag::error::{Result, TagError};
use crate::tag::fetch::Tag;
use crate::tag::{TagAuto, TagAutoCompare, TagAutoSelect, TagType};

/// List Projects
///
/// - Alternative route: `/latest/projects`
/// - Alternative route: `/v1/projects`
///
/// ---
///
/// Lists all projects the user has access to.
///
/// ## Security
/// - authenticated
/// - project_group:read
#[utoipa::path(
    get,
    path = "/",
    tag = "projects",
    params(TagFilter),
    responses(
        (
            body = Vec<Tag>,
            description = "List all projects that match the given filters",
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
    Query(filter): Query<TagFilter>,
) -> Result<impl IntoResponse> {
    let data = list(&state.postgres, identity.character_id, filter).await?;

    if data.is_empty() {
        Ok((StatusCode::NO_CONTENT, Json(data)).into_response())
    } else {
        Ok((StatusCode::OK, Json(data)).into_response())
    }
}

pub async fn list(
    pool: &PgPool,
    character_id: CharacterId,
    filter: TagFilter,
) -> Result<Vec<Tag>> {
    let entries = sqlx::query!(
        r#"
            SELECT
                id,
                content,
                color,
                typ
            FROM tag
            WHERE
                owner_id = $1
            ORDER BY content
        "#,
        *character_id,
    )
    .fetch_all(pool)
    .await
    .map_err(TagError::List)?
    .into_iter()
    .filter(|x| {
        // unwrap is safe, as it's set per default
        if filter
            .auto
            .unwrap()
            && x.typ == "AUTO"
        {
            true
        } else {
            filter
                .manual
                .unwrap()
                && x.typ == "MANUAL"
        }
    })
    .collect::<Vec<_>>();

    let mut results = Vec::new();
    for entry in entries {
        let typ = match TagType::try_from(entry.typ) {
            Ok(x) => x,
            Err(e) => {
                tracing::warn!("{}", e.to_string());
                continue
            },
        };

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
                entry.id,
            )
            .fetch_all(pool)
            .await
            .map_err(|e| TagError::Fetch(e, entry.id.into()))?;

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
            id:      entry.id.into(),
            color:   entry.color,
            content: entry.content,
            typ:     typ,
            auto:    triggers,
        };
        results.push(tag);
    }

    Ok(results)
}

#[derive(Debug, Default, Deserialize, Serialize, ToSchema, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct TagFilter {
    #[serde(default = "default_true")]
    #[param(
        default = json!(true),
        required = false,
    )]
    pub auto: Option<bool>,

    #[serde(default = "default_true")]
    #[param(
        default = json!(true),
        required = false,
    )]
    pub manual: Option<bool>,
}

fn default_true() -> Option<bool> {
    Some(true)
}
