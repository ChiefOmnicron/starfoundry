use sqlx::PgPool;
use starfoundry_lib_industry::TagUuid;
use starfoundry_lib_types::CharacterId;
use starfoundry_lib_industry::tag::{CreateTag, TagType};

use crate::tag::error::{TagError, Result};

pub async fn create(
    pool:           &PgPool,
    character_id:   CharacterId,
    tag_info:       CreateTag,
) -> Result<TagUuid> {
    tag_info.validate()?;

    let mut transaction = pool
        .begin()
        .await
        .map_err(TagError::TransactionError)?;
    let tag_id = sqlx::query!(r#"
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
            &tag_info.typ.to_string(),
        )
        .fetch_one(&mut *transaction)
        .await
        .map_err(TagError::Create)?;

    if tag_info.typ == TagType::Auto {
        let mut options = Vec::new();
        let mut compares = Vec::new();
        let mut values = Vec::new();

        for auto_tag in tag_info.auto {
            options.push(auto_tag.select.to_string());
            compares.push(auto_tag.compare.to_string());
            values.push(auto_tag.value);
        }

        sqlx::query!("
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
    Ok(tag_id.id.into())
}
