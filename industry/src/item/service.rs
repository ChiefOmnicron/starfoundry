use serde::Serialize;
use sqlx::PgPool;
use starfoundry_lib_types::{CategoryId, GroupId, TypeId};
use utoipa::ToSchema;

use crate::item::{ItemError, Result};

#[derive(Debug, Serialize, ToSchema)]
#[cfg_attr(test, derive(serde::Deserialize))]
#[schema(
    example = json!({
        "base_price": null,
        "category_id": 6,
        "group_id": 30,
        "meta_group_id": null,
        "name": "Ragnarok",
        "repackaged": 10000000,
        "type_id": 23773,
        "volume": 100000000
    })
)]
pub struct Item {
    pub category_id:   CategoryId,
    pub group_id:      GroupId,
    pub name:          String,
    pub type_id:       TypeId,
    pub volume:        f32,

    pub meta_group_id: Option<GroupId>,
    pub repackaged:    Option<i32>,
}

impl Item {
    pub async fn new(
        pool:    &PgPool,
        type_id: TypeId,
    ) -> Result<Option<Self>> {
        let item = sqlx::query!("
                SELECT
                    type_id,
                    category_id,
                    group_id,
                    volume,
                    name,
                    meta_group_id,
                    repackaged
                FROM item
                WHERE type_id = $1
                ORDER BY name
            ",
                *type_id,
            )
            .fetch_optional(pool)
            .await
            .map_err(|e| ItemError::FetchItem(e, type_id))?;

        if let Some(x) = item {
            Ok(Some(Item {
                category_id:   x.category_id.into(),
                group_id:      x.group_id.into(),
                name:          x.name,
                type_id:       x.type_id.into(),
                volume:        x.volume,

                meta_group_id: x.meta_group_id.map(Into::into),
                repackaged:    x.repackaged,
            }))
        } else {
            Ok(None)
        }
    }
}
