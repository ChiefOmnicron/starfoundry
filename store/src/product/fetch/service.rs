use sqlx::PgPool;
use starfoundry_lib_types::{AllianceId, CharacterId, CorporationId};

use crate::config::ProductUuid;
use crate::product::{Product, ProductError, Result};
use crate::product::create::AdditionalOption;

pub async fn fetch(
    pool: &PgPool,
    product_uuid: ProductUuid,
    character_id: CharacterId,
    corporation_id: CorporationId,
    alliance_id: Option<AllianceId>,
) -> Result<Option<Product>> {
    let product = sqlx::query!("
            SELECT
                category,
                name,
                price,
                image_type,
                image_type_id,
                description,
                tags,
                content,
                message,
                delivery_time,
                additional_products,
                blacklist,
                whitelist
            FROM product
            WHERE id = $1
        ",
            *product_uuid,
        )
        .fetch_optional(pool)
        .await
        .map_err(ProductError::GeneralSqlxError)?;

    if let Some(x) = product {
        let tags = x.tags.unwrap_or_default();

        #[allow(unused_parens)]
        if (
            x.blacklist.contains(&*character_id) ||
            x.blacklist.contains(&*corporation_id) ||
            x.blacklist.contains(&*alliance_id.unwrap_or(0.into()))
        ) {
            return Ok(None)
        }

        if !x.whitelist.is_empty() {
            if !(
                x.whitelist.contains(&*character_id) ||
                x.whitelist.contains(&*corporation_id) ||
                x.whitelist.contains(&*alliance_id.unwrap_or(0.into()))
            ) {
                return Ok(None)
            }
        }

        let product = Product {
            id: product_uuid,
            description: x.description,
            category: x.category,
            image_type: x.image_type,
            image_type_id: x.image_type_id.into(),
            name: x.name,
            price: x.price as u64,
            tags: tags,
            content: x.content,
            message: x.message,
            delivery_time: x.delivery_time,
            additional_options: x.additional_products
                .unwrap_or_default()
                .into_iter()
                .map(|x| AdditionalOption {
                    reference_id: x.into(),
                })
                .collect::<Vec<_>>(),
        };

        Ok(Some(product))
    } else {
        Ok(None)
    }
}
