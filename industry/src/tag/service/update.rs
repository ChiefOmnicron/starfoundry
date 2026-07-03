use sqlx::PgPool;
use starfoundry_lib_industry::tag::{TagType, UpdateTag};
use starfoundry_lib_industry::TagUuid;
use starfoundry_lib_types::CharacterId;

use crate::tag::error::{Result, TagError};

pub async fn update(
    pool:           &PgPool,
    character_id:   CharacterId,
    tag_id:         TagUuid,
    tag_info:       UpdateTag,
) -> Result<()> {
    tag_info.validate()?;

    let mut transaction = pool
        .begin()
        .await
        .map_err(TagError::TransactionError)?;
    sqlx::query!(r#"
            DELETE FROM tag_auto
            WHERE tag_id = $1
        "#,
            *tag_id,
        )
        .execute(&mut *transaction)
        .await
        .map(drop)
        .map_err(TagError::Update)?;

    sqlx::query!("
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
            tag_info.typ.to_string(),
        )
        .execute(&mut *transaction)
        .await
        .map_err(TagError::Update)?;

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
