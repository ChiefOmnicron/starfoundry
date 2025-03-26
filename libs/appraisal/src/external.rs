mod janice;

pub use self::janice::*;

use starfoundry_libs_types::TypeId;
use std::fmt;

use crate::Error;

/// Generalized trait for communicating with appraisal site.
#[async_trait::async_trait]
pub trait ExternalAppraisal<T> {
    /// Validates that all required Environment variables are set
    fn validate() -> Result<(), Error>;

    /// Creates a new appraisal
    /// 
    /// # Params
    /// 
    /// * `persist` > Determines if the apprials should be saved
    /// * `entries` > List of all items that should be appraisald
    /// 
    async fn create(
        &self,
        persist: Persistance,
        entries: Vec<AppraisalEntry>,
    ) -> Result<T, Error>;
}

pub enum Persistance {
    Persist,
    NonPersistent,
}

impl fmt::Display for Persistance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Persist       => write!(f, "true"),
            Self::NonPersistent => write!(f, "false"),
        }
    }
}

#[derive(Debug)]
pub struct AppraisalEntry {
    pub name:     String,
    pub type_id:  TypeId,
    pub quantity: i32,
}
