use sqlx::PgPool;
use starfoundry_lib_types::CharacterId;
use starfoundry_lib_industry::tag::{Tag, TagAuto, TagAutoCompare, TagAutoSelect, TagType};

use crate::tag::error::{Result, TagError};

pub async fn list(
    pool:           &PgPool,
    character_id:   CharacterId,
) -> Result<Vec<Tag>> {
    let entries = sqlx::query!(r#"
            SELECT
                id,
                content,
                color,
                typ
            FROM tag
            WHERE owner_id = $1
        "#,
            *character_id,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| TagError::List(e))?;

    let mut results = Vec::new();
    for entry in entries {
        let typ = match TagType::try_from(entry.typ) {
            Ok(x)  => x,
            Err(e) => {
                tracing::warn!("{}", e.to_string());
                continue
            }
        };

        let triggers = if typ == TagType::Auto {
            let triggers = sqlx::query!("
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
            id:         entry.id.into(),
            color:      entry.color,
            content:    entry.content,
            typ:        typ,
            auto:       triggers,
        };
        results.push(tag);
    }

    Ok(results)
}
