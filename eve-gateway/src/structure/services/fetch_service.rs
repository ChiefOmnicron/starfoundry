use sqlx::PgPool;
use starfoundry_lib_eve_gateway::StructureServiceResponse;
use starfoundry_lib_types::TypeId;

use crate::structure::error::{Result, StructureError};
use crate::item::services::fetch_item_bulk;

pub async fn fetch_services(
    pool:              &PgPool,
    structure_type_id: TypeId,
) -> Result<StructureServiceResponse> {
    let services = sqlx::query!(r#"
            SELECT service_type_ids, service_slots
            FROM structure_service
            WHERE structure_type_id = $1
        "#,
            *structure_type_id,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| StructureError::FetchStructureServices(e, structure_type_id))?;

    let type_ids = services
        .iter()
        .map(|x| x.service_type_ids.clone())
        .flat_map(|x| x.into_iter().map(Into::into).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let items = fetch_item_bulk(
            pool,
            type_ids,
        )
        .await?;

    Ok(StructureServiceResponse {
        slots:    services.first().map(|x| x.service_slots).unwrap_or_default(),
        services: items,
    })
}
