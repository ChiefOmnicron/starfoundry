use regex::Regex;
use sqlx::PgPool;
use starfoundry_lib_industry::{ProjectUuid, TagUuid};
use starfoundry_lib_industry::project::{ProjectFilter, ProjectMinimal};
use starfoundry_lib_industry::tag::{Tag, TagAutoCompare, TagAutoSelect};
use starfoundry_lib_types::CharacterId;

use crate::tag::error::{Result, TagError};
use crate::tag::service::list;
use crate::tag::list::TagFilter;

pub async fn trigger(
    pool:           &PgPool,
    character_id:   CharacterId,
) -> Result<()> {
    let tags = list(pool, character_id, TagFilter {
        auto:   Some(true),
        manual: Some(false),
    }).await?;

    let projects = crate::project::service::list(
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

    sqlx::query!("
            SELECT *
            FROM project_tag
        ")
        .fetch_all(pool)
        .await
        .map_err(TagError::SqlxError)?
        .into_iter()
        .for_each(|x| {
            if tag_updates
                .iter()
                .find(|y| y.project_id == x.project_id.into() && y.tag_id == x.tag_id.into())
                .is_none() {

                tag_updates.push(TmpTag {
                    project_id: x.project_id.into(),
                    tag_id:     x.tag_id.into(),
                })
            }
        });

    if !tag_updates.is_empty() {
        sqlx::query!("
                DELETE FROM project_tag
            ")
            .execute(pool)
            .await
            .map_err(TagError::SqlxError)?;

        sqlx::query!("
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
                &tag_updates.iter().map(|x| *x.project_id).collect::<Vec<_>>(),
                &tag_updates.iter().map(|x| *x.tag_id).collect::<Vec<_>>()
            )
            .execute(pool)
            .await
            .map_err(TagError::SqlxError)?;
    }

    Ok(())
}

fn is_valid(
    tag:        &Tag,
    project:    &ProjectMinimal,
) -> bool {
    let mut valid = true;

    for auto_tag in tag.auto.iter() {
        if !valid {
            break;
        }

        valid = match auto_tag.select {
            TagAutoSelect::ProjectName      => {
                match_compare(
                    &auto_tag.compare,
                    &project.name,
                    &auto_tag.value,
                )
            },
            TagAutoSelect::ProjectOrderer   => {
                match_compare(
                    &auto_tag.compare,
                    &project.orderer,
                    &auto_tag.value,
                )
            },
            TagAutoSelect::ProjectStatus    => {
                match_compare(
                    &auto_tag.compare,
                    project.status.as_str(),
                    &auto_tag.value,
                )
            },
        }
    }

    valid
}

fn match_compare(
    compare:        &TagAutoCompare,
    actual_value:   &str,
    target_value:   &str,
) -> bool {
    match compare {
        TagAutoCompare::Is          => {
            actual_value == target_value
        },
        TagAutoCompare::IsNot       => {
            actual_value != target_value
        },
        TagAutoCompare::Contains    => {
            actual_value.contains(target_value)
        },
        TagAutoCompare::Pattern     => {
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
