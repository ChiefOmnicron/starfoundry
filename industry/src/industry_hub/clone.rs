use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Serialize;
use sqlx::PgPool;
use starfoundry_gateway::ExtractIdentity;
use starfoundry_lib_types::CharacterId;
use utoipa::ToSchema;

use crate::api_docs::{Forbidden, InternalServerError, Unauthorized};
use crate::industry_hub::IndustryHubError;
use crate::industry_hub::error::Result;
use crate::{AppState, IndustryHubUuid};

/// Clone Industry Hub
///
/// - Alternative route: `/latest/industry-hubs/{IndustryHubUuid}/clone`
/// - Alternative route: `/v1/industry-hubs/{IndustryHubUuid}/clone`
///
/// ---
///
/// Clones a industry hub
///
/// ## Security
/// - authenticated
/// - industry_hub:shared
#[utoipa::path(
    put,
    path = "/{IndustryHubUuid}/clone",
    tag = "Industry Hubs",
    params(
        IndustryHubUuid,
    ),
    responses(
        (
            body = IndustryHubCloneResponse,
            description = "The structure group was successfully cloned",
            status = OK
        ),
        Unauthorized,
        Forbidden,
        InternalServerError,
    ),
    security(
        ("api_key" = [])
    ),
)]
pub async fn api(
    identity: ExtractIdentity,
    State(state): State<AppState>,
    Path(industry_hub_uuid): Path<IndustryHubUuid>,
) -> Result<impl IntoResponse> {
    let data = clone(&state.postgres, identity.character_id, industry_hub_uuid)
        .await?;

    Ok((
        StatusCode::NO_CONTENT,
        Json(IndustryHubCloneResponse {
            id: data,
        }),
    )
        .into_response())
}

pub async fn clone(
    pool: &PgPool,
    character_id: CharacterId,
    industry_hub_id: IndustryHubUuid,
) -> Result<IndustryHubUuid> {
    let mut transaction = pool
        .begin()
        .await
        .map_err(IndustryHubError::TransactionError)?;

    let cloned_id: IndustryHubUuid = sqlx::query!(
        "
            INSERT INTO industry_hub
            (
                owner,
                name,
                description
            )
            VALUES(
                $1,
                (SELECT name FROM industry_hub WHERE id = $2),
                (SELECT description FROM industry_hub WHERE id = $2)
            )
            RETURNING id
        ",
        *character_id,
        *industry_hub_id,
    )
    .fetch_one(&mut *transaction)
    .await
    .map(|x| IndustryHubUuid::new(x.id))
    .map_err(IndustryHubError::CreateIndustryHub)?;

    let structure_ids = sqlx::query!(
        "
            INSERT INTO structure
            (
                owner,
                structure_id,
                system_id,
                type_id,
                rigs,
                services,
                name,
                x,
                y,
                z
            )
            SELECT
                $1 AS owner,
                s.structure_id,
                s.system_id,
                s.type_id,
                s.rigs,
                s.services,
                s.name,
                s.x,
                s.y,
                s.z
            FROM industry_hub_structure ihs
            JOIN structure s ON ihs.structure_id = s.id
            WHERE ihs.industry_hub_id = $2
            RETURNING id
        ",
        *character_id,
        *industry_hub_id,
    )
    .fetch_all(&mut *transaction)
    .await
    .map_err(|e| IndustryHubError::CloneIndustryHub(e, industry_hub_id))?
    .into_iter()
    .map(|x| x.id)
    .collect::<Vec<_>>();

    sqlx::query!(
        "
            INSERT INTO industry_hub_structure
            (
                industry_hub_id,
                structure_id
            )
            SELECT $1, * FROM UNNEST($2::UUID[])
        ",
        *cloned_id,
        &structure_ids,
    )
    .execute(&mut *transaction)
    .await
    .map_err(|e| IndustryHubError::UpdateIndustryHub(e, cloned_id))?;

    sqlx::query!(
        "
            INSERT INTO industry_hub_clone
            (
                original_id,
                new_id
            )
            VALUES ($1, $2)
        ",
        *industry_hub_id,
        *cloned_id,
    )
    .execute(&mut *transaction)
    .await
    .map_err(IndustryHubError::CreateIndustryHub)?;

    transaction
        .commit()
        .await
        .map_err(IndustryHubError::TransactionError)?;

    Ok(cloned_id)
}

#[derive(Debug, Serialize, ToSchema)]
pub struct IndustryHubCloneResponse {
    id: IndustryHubUuid,
}
