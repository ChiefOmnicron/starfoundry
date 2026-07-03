use sqlx::PgPool;
use starfoundry_lib_industry::TagUuid;
use starfoundry_lib_types::CharacterId;
use starfoundry_lib_industry::tag::{Tag, TagAuto, TagAutoCompare, TagAutoSelect, TagType};

use crate::tag::error::{Result, TagError};

pub async fn fetch(
    pool:           &PgPool,
    character_id:   CharacterId,
    tag_id:         TagUuid,
) -> Result<Option<Tag>> {
    let entry = sqlx::query!(r#"
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
            let triggers = sqlx::query!("
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
                    Ok(x)  => x,
                    Err(e) => {
                        tracing::warn!("{}", e.to_string());
                        continue
                    }
                };
                let compare = match TagAutoCompare::try_from(trigger.compare) {
                    Ok(x)  => x,
                    Err(e) => {
                        tracing::warn!("{}", e.to_string());
                        continue
                    }
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
            id:         x.id.into(),
            color:      x.color,
            content:    x.content,
            typ:        typ,
            auto:       triggers,
        };
        Ok(Some(tag))
    } else {
        Ok(None)
    }
}
