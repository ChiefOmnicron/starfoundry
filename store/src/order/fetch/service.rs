use sqlx::PgPool;

use crate::config::ProductUuid;
use crate::product::{Product, ProductError, Result};
use crate::product::create::AdditionalOption;

pub async fn fetch(
    pool: &PgPool,
    product_uuid: ProductUuid,
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
                additional_products
            FROM product
            WHERE uuid = $1
        ",
            *product_uuid,
        )
        .fetch_optional(pool)
        .await
        .map_err(ProductError::GeneralSqlxError)?;

    if let Some(x) = product {
        let mut tags = x.tags.unwrap_or_default();
        tags.sort();

        let product = Product {
            description: x.description,
            category: x.category,
            image_type: x.image_type,
            image_type_id: x.image_type_id.into(),
            name: x.name,
            price: x.price as u64,
            tags: tags,
            uuid: product_uuid,
            content: x.content,
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
