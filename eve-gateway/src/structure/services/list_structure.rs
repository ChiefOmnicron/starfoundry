use sqlx::PgPool;
use starfoundry_lib_eve_gateway::StructureServiceResponse;
use starfoundry_lib_types::TypeId;

use crate::item::services::fetch_item_bulk;
use crate::structure::error::{Result, StructureError};

pub async fn list_structure_services(
    pool:              &PgPool,
    structure_type_id: TypeId,
) -> Result<Option<StructureServiceResponse>> {
    let services = sqlx::query!(r#"
            SELECT
                service_type_ids,
                service_slots
            FROM structure_service
            WHERE structure_type_id = $1
        "#,
            *structure_type_id,
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| StructureError::FetchStructureServices(e, structure_type_id))?;

    let services = if let Some(x) = services {
        x
    } else {
        return Ok(None);
    };

    let type_ids = services
        .service_type_ids
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<_>>();

    let items = fetch_item_bulk(
            pool,
            type_ids,
        )
        .await?;

    Ok(Some(StructureServiceResponse {
        services: items,
        slots: services.service_slots,
    }))
}
