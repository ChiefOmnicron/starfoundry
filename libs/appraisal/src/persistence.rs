use serde::Deserialize;
use std::fmt;
use utoipa::ToSchema;

#[derive(Clone, Debug, Deserialize, ToSchema)]
pub enum Persistance {
    Persist,
    NonPersist,
}

impl Persistance {
    pub fn is_persistent(
        &self,
    ) -> bool {
        match self {
            Persistance::Persist    => true,
            Persistance::NonPersist => false,
        }
    }
}

impl fmt::Display for Persistance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Persist    => write!(f, "Persist"),
            Self::NonPersist => write!(f, "NonPersist"),
        }
    }
}
