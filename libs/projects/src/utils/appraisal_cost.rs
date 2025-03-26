use serde::Deserialize;
use starfoundry_libs_appraisal::{ExternalAppraisal, AppraisalEntry, JaniceAppraisal, Persistance};
use starfoundry_libs_appraisal::internal::InternalAppraisal;
use starfoundry_libs_types::TypeId;
use std::collections::HashMap;
use utoipa::ToSchema;

use crate::{Error, Result};
use sqlx::PgPool;

pub async fn appraisal(
    pool:      &PgPool,
    appraisal: AppraisalList,
    entries:   Vec<AppraisalEntry>,
) -> Result<HashMap<TypeId, f32>> {
    match appraisal {
        AppraisalList::Internal => internal(&pool, entries).await,
        AppraisalList::Janice   => janice(entries).await,
    }
}

async fn internal(
    pool: &PgPool,
    entries: Vec<AppraisalEntry>,
) -> Result<HashMap<TypeId, f32>> {
    let entries = InternalAppraisal::new(pool.clone())
        .map_err(Error::AppraisalError)?
        .create(Persistance::NonPersistent, entries)
        .await
        .map_err(Error::AppraisalError)?
        .items
        .into_iter()
        .map(|x| (x.type_id.into(), x.total_sell as f32))
        .collect::<HashMap<_, _>>();
    Ok(entries)
}

async fn janice(
    entries: Vec<AppraisalEntry>,
) -> Result<HashMap<TypeId, f32>> {
    let entries = JaniceAppraisal::new()
        .map_err(Error::AppraisalError)?
        .create(Persistance::NonPersistent, entries)
        .await
        .map_err(Error::AppraisalError)?
        .items
        .into_iter()
        .map(|x| (x.item_type.eid.into(), x.immediate_price.sell_price_total))
        .collect::<HashMap<_, _>>();
    Ok(entries)
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AppraisalList {
    Internal,

    /// only available if the feature flag is set and an API key is set
    Janice,
}
