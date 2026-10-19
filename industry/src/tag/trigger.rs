use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use regex::Regex;
use sqlx::PgPool;
use starfoundry_gateway::ExtractIdentity;
use starfoundry_lib_types::CharacterId;

use crate::api_docs::{
    Forbidden, InternalServerError, NotFound, Unauthorized, UnprocessableEntity, UnsupportedMediaType
};
use crate::project::list::{ProjectFilter, ProjectMinimal};
use crate::tag::error::{Result, TagError};
use crate::tag::fetch::Tag;
use crate::tag::list::{TagFilter, list};
use crate::tag::update::UpdateTag;
use crate::tag::{TagAutoCompare, TagAutoSelect};
use crate::{AppState, ProjectUuid, TagUuid};

/// Trigger
///
/// - Alternative route: `/v1/tags`
/// - Alternative route: `/latest/tags`
///
/// ---
///
/// Triggers the auto tag generation
///
/// ## Security
/// - authenticated
/// - project_group:write
#[utoipa::path(
    put,
    path = "/",
    tag = "Tags",
    request_body = UpdateTag,
    params(
        TagUuid,
    ),
    responses(
        (
            description = "Tags were updated",
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
) -> Result<impl IntoResponse> {
    trigger(&state.postgres, identity.character_id).await?;

    Ok((StatusCode::NO_CONTENT,))
}

pub async fn trigger(
    pool: &PgPool,
    character_id: CharacterId,
) -> Result<()> {
    let tags = list(
        pool,
        character_id,
        TagFilter {
            auto:   Some(true),
            manual: Some(false),
        },
    )
    .await?;

    let projects = crate::project::list::list(
        pool,
        character_id,
        ProjectFilter::default(),
    )
    .await?;

    let mut tag_updates = Vec::new();

    for tag in tags.iter() {
        for project in projects.iter() {
            if is_valid(tag, project) {
                tag_updates.push(TmpTag {
                    project_id: project.id,
                    tag_id:     tag.id,
                });
            }
        }
    }

    sqlx::query!(
        "
            SELECT *
            FROM project_tag
        "
    )
    .fetch_all(pool)
    .await
    .map_err(TagError::SqlxError)?
    .into_iter()
    .for_each(|x| {
        if tag_updates
            .iter()
            .find(|y| {
                y.project_id
                    == x.project_id
                        .into()
                    && y.tag_id == x.tag_id.into()
            })
            .is_none()
        {
            tag_updates.push(TmpTag {
                project_id: x
                    .project_id
                    .into(),
                tag_id:     x.tag_id.into(),
            })
        }
    });

    if !tag_updates.is_empty() {
        sqlx::query!(
            "
                DELETE FROM project_tag
            "
        )
        .execute(pool)
        .await
        .map_err(TagError::SqlxError)?;

        sqlx::query!(
            "
                INSERT INTO project_tag
                (
                    project_id,
                    tag_id
                )
                SELECT * FROM UNNEST(
                    $1::UUID[],
                    $2::UUID[]
                )
            ",
            &tag_updates
                .iter()
                .map(|x| *x.project_id)
                .collect::<Vec<_>>(),
            &tag_updates
                .iter()
                .map(|x| *x.tag_id)
                .collect::<Vec<_>>()
        )
        .execute(pool)
        .await
        .map_err(TagError::SqlxError)?;
    }

    Ok(())
}

fn is_valid(
    tag: &Tag,
    project: &ProjectMinimal,
) -> bool {
    let mut valid = true;

    for auto_tag in tag.auto.iter() {
        if !valid {
            break;
        }

        valid = match auto_tag.select {
            TagAutoSelect::ProjectName => {
                match_compare(&auto_tag.compare, &project.name, &auto_tag.value)
            },
            TagAutoSelect::ProjectOrderer => {
                match_compare(
                    &auto_tag.compare,
                    &project.orderer,
                    &auto_tag.value,
                )
            },
            TagAutoSelect::ProjectStatus => {
                match_compare(
                    &auto_tag.compare,
                    project
                        .status
                        .as_str(),
                    &auto_tag.value,
                )
            },
        }
    }

    valid
}

fn match_compare(
    compare: &TagAutoCompare,
    actual_value: &str,
    target_value: &str,
) -> bool {
    match compare {
        TagAutoCompare::Is => actual_value == target_value,
        TagAutoCompare::IsNot => actual_value != target_value,
        TagAutoCompare::Contains => actual_value.contains(target_value),
        TagAutoCompare::Pattern => {
            if let Ok(x) = Regex::new(target_value) {
                x.is_match(actual_value)
            } else {
                false
            }
        },
    }
}

struct TmpTag {
    tag_id:     TagUuid,
    project_id: ProjectUuid,
}
