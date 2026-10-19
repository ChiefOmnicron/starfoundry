use sqlx::PgPool;

use crate::ProjectUuid;
use crate::project::error::{ProjectError, Result};
use crate::tag::TagType;
use crate::tag::fetch::Tag;

pub async fn list_tags(
    pool: &PgPool,
    project_id: ProjectUuid,
) -> Result<Vec<Tag>> {
    let tags = sqlx::query!(
        "
            SELECT
                t.id,
                t.color,
                t.content,
                t.typ
            FROM project_tag pt
            JOIN tag t ON pt.tag_id = t.id
            WHERE
                pt.project_id = $1
        ",
        *project_id,
    )
    .fetch_all(pool)
    .await
    .map_err(|e| ProjectError::Fetch(e, project_id))?;

    let mut result = Vec::new();
    for tag in tags {
        let typ = if let Ok(x) = TagType::try_from(tag.typ) {
            x
        } else {
            continue
        };

        let tag = Tag {
            id:      tag.id.into(),
            color:   tag.color,
            content: tag.content,
            typ:     typ,
            auto:    Vec::new(),
        };
        result.push(tag);
    }

    Ok(result)
}
