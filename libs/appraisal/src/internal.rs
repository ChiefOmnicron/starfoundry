use sqlx::PgPool;
use starfoundry_lib_types::TypeId;
use std::collections::HashMap;

use crate::{ExternalAppraisal, AppraisalEntry, Persistance, Result};

mod compression;
mod create;
mod fetch;
mod models;
mod reprocessing;

pub use self::compression::*;
pub use self::create::*;
pub use self::fetch::*;
pub use self::models::*;
pub use self::reprocessing::*;

pub struct InternalAppraisal(PgPool);

impl InternalAppraisal {
    pub fn new(
        pool: PgPool,
    ) -> Self {
        Self(pool)
    }
}

#[async_trait::async_trait]
impl ExternalAppraisal<Appraisal> for InternalAppraisal
where
    Self: Sized,
{
    fn validate() -> Result<()> {
        Ok(())
    }

    async fn create(
        &self,
        _persist: Persistance,
        entries:  Vec<AppraisalEntry>,
    ) -> Result<Appraisal> {
        let entries = entries
            .into_iter()
            .map(|x| (x.type_id, x.quantity as i64))
            .collect::<HashMap<TypeId, i64>>();

        let mut options = AppraisalOptions::default();
        options.set_persist(Some(Persistance::NonPersist));

        crate::internal::create_type_ids(
            &self.0,
            entries,
            Some(options),
        )
        .await
    }
}

#[derive(Clone, Debug, Default)]
pub struct MarketPriceEntry {
    pub type_id:   i32,
    pub item_name: String,
    pub remaining: i32,
    pub price:     f64,
    pub quantity:  i32,
}
