use serde::{Deserialize, Deserializer};
use std::fmt;
use utoipa::ToSchema;

#[derive(Clone, Debug, Deserialize, ToSchema)]
pub enum Persistance {
    Persist,
    NonPersistent,
}

impl Persistance {
    pub fn is_persistent(
        &self,
    ) -> bool {
        match self {
            Persistance::Persist       => true,
            Persistance::NonPersistent => false,
        }
    }

    pub fn deserialize<'de, D>(
        deserializer: D
    ) -> Result<Option<Self>, D::Error>
        where
            D: Deserializer<'de>,
    {
        match Deserialize::deserialize(deserializer)? {
            true =>  Ok(Some(Self::Persist)),
            false => Ok(Some(Self::NonPersistent)),
        }
    }
}

impl fmt::Display for Persistance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Persist       => write!(f, "true"),
            Self::NonPersistent => write!(f, "false"),
        }
    }
}
