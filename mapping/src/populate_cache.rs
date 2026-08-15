use starfoundry_lib_eve_gateway::{EveGatewayApiClientSystem, EveGatewayClient};
use sqlx::PgPool;

use crate::SERVICE_NAME;

pub async fn populate_system(
    pool: &PgPool,
) -> Result<(), Box<dyn std::error::Error>> {
    let entries = sqlx::query!("
            SELECT COUNT(1) FROM system_cache
        ")
        .fetch_one(pool)
        .await?;

    if entries.count.unwrap_or_default() > 0 {
        return Ok(())
    }

    tracing::info!("Populating system cache");
    let eve_gateway_client = EveGatewayClient::new(SERVICE_NAME)?;
    let systems = eve_gateway_client
        .list_systems()
        .await?;

    sqlx::query!("
            INSERT INTO system_cache (
                region_id,
                constellation_id,
                system_id,

                region_name,
                constellation_name,
                system_name,

                security,
                security_str
            )
            SELECT * FROM UNNEST(
                $1::INTEGER[],
                $2::INTEGER[],
                $3::INTEGER[],

                $4::VARCHAR[],
                $5::VARCHAR[],
                $6::VARCHAR[],

                $7::REAL[],
                $8::VARCHAR[]
            )
        ",
            &systems.iter().map(|x| *x.region_id).collect::<Vec<_>>(),
            &systems.iter().map(|x| *x.constellation_id).collect::<Vec<_>>(),
            &systems.iter().map(|x| *x.system_id).collect::<Vec<_>>(),

            &systems.iter().map(|x| x.region_name.clone()).collect::<Vec<_>>(),
            &systems.iter().map(|x| x.constellation_name.clone()).collect::<Vec<_>>(),
            &systems.iter().map(|x| x.system_name.clone()).collect::<Vec<_>>(),

            &systems.iter().map(|x| x.security).collect::<Vec<_>>(),
            &systems.iter().map(|x| x.security_str.clone()).collect::<Vec<_>>(),
        )
        .execute(pool)
        .await?;

    tracing::info!("System cache populated");
    Ok(())
}

pub async fn populate_system_distance(
    pool: &PgPool,
) -> Result<(), Box<dyn std::error::Error>> {
    let now = std::time::Instant::now();
    let entries = sqlx::query!("
            SELECT COUNT(1) FROM system_distance_cache
        ")
        .fetch_one(pool)
        .await?;

    if entries.count.unwrap_or_default() > 0 {
        return Ok(())
    }

    tracing::info!("Populating system distance cache");
    let eve_gateway_client = EveGatewayClient::new(SERVICE_NAME)?;
    let systems = eve_gateway_client
        .list_distances()
        .await?;

    sqlx::query!("
            INSERT INTO system_distance_cache (
                system_start,
                system_end,
                distance_ly
            )
            SELECT * FROM UNNEST(
                $1::INTEGER[],
                $2::INTEGER[],
                $3::REAL[]
            )
        ",
            &systems.iter().map(|x| *x.system_start).collect::<Vec<_>>(),
            &systems.iter().map(|x| *x.system_end).collect::<Vec<_>>(),
            &systems.iter().map(|x| *x.distance_ly).collect::<Vec<_>>(),
        )
        .execute(pool)
        .await?;

    dbg!(now.elapsed().as_secs());
    tracing::info!("System distance cache populated");
    Ok(())
}
