use sqlx::PgPool;
use starfoundry_lib_types::CharacterId;
use starfoundry_lib_industry::route::{CreateRoute, RouteType};
use crate::route::error::{Result, RouteError};
use starfoundry_lib_industry::RouteUuid;
use uuid::Uuid;

pub async fn create(
    pool:           &PgPool,
    character_id:   CharacterId,
    route_info:     CreateRoute,
) -> Result<RouteUuid> {
    route_info.validate()?;

    let mut transaction = pool
        .begin()
        .await
        .map_err(RouteError::TransactionError)?;

    let route_id = sqlx::query!("
            INSERT INTO route (
                name,
                typ,
                start_structure_id,
                end_structure_id
            )
            VALUES ($1, $2, $3, $4)
            RETURNING id
        ",
            route_info.name,
            route_info.typ.as_str(),
            *route_info.start_structure,
            *route_info.end_structure,
        )
        .fetch_one(pool)
        .await
        .map(|x| x.id)
        .map_err(RouteError::Create)?;

    if let Some(x) = route_info.jump_route {
        sqlx::query!("
                INSERT INTO route_jump_route (
                    route_id,
                    fuel_usage
                )
                VALUES ($1, $2)
            ",
                route_id,
                x.fuel_usage,
            )
            .execute(pool)
            .await
            .map_err(RouteError::Create)?;
    }

    if let Some(x) = route_info.hauling_route {
        sqlx::query!("
                INSERT INTO route_hauling_route (
                    route_id,
                    fuel_usage,
                    max_cargo_m3
                )
                VALUES ($1, $2, $3)
            ",
                route_id,
                x.fuel_usage,
                x.max_cargo_m3,
            )
            .execute(pool)
            .await
            .map_err(RouteError::Create)?;
    }

    if let Some(x) = route_info.hauling_service {
        sqlx::query!("
                INSERT INTO route_hauling_service (
                    route_id,
                    contract_to,
                    price_per_m3,
                    max_cargo_m3,
                    collateral_percent
                )
                VALUES ($1, $2, $3, $4, $5)
            ",
                route_id,
                x.contract_to,
                x.price_per_m3,
                x.max_cargo_m3,
                x.collateral_percent,
            )
            .execute(pool)
            .await
            .map_err(RouteError::Create)?;
    }

    transaction
        .commit()
        .await
        .map_err(RouteError::TransactionError)?;

    Ok(RouteUuid::new(Uuid::now_v7()))
}
