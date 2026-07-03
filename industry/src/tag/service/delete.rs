use sqlx::PgPool;
use starfoundry_lib_industry::TagUuid;

use crate::tag::error::{Result, TagError};

pub async fn delete(
    pool:   &PgPool,
    tag_id: TagUuid,
) -> Result<()> {
    sqlx::query!(r#"
            DELETE FROM tag
            WHERE id = $1
        "#,
            *tag_id,
        )
        .execute(pool)
        .await
        .map(drop)
        .map_err(|e| TagError::Delete(e, tag_id))
}
