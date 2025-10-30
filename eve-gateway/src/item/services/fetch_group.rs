use sqlx::PgPool;
use starfoundry_lib_eve_gateway::Group;
use starfoundry_lib_types::GroupId;

use crate::item::error::{ItemError, Result};

pub async fn fetch_group(
    pool:     &PgPool,
    group_id: GroupId,
) -> Result<Option<Group>> {
    let category = sqlx::query!(r#"
            SELECT
                group_id,
                category_id,
                name
            FROM groups
            WHERE group_id = $1
        "#,
            *group_id,
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| ItemError::FetchGroup(e, group_id))?;

    if let Some(x) = category {
        Ok(Some(Group {
            group_id:    x.group_id.into(),
            category_id: x.category_id.into(),
            name:        x.name,
        }))
    } else {
        Ok(None)
    }
}
