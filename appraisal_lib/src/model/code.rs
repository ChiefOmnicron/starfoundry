use rand::distr::Alphanumeric;
use rand::RngExt;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::Deref;
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Deserialize, Serialize, IntoParams, ToSchema)]
#[into_params(names("code"))]
pub struct AppraisalCode(String);

impl AppraisalCode {
    pub fn new() -> Self {
        let mut rng = rand::rng();
        let code = (&mut rng).sample_iter(Alphanumeric)
            .take(10)
            .map(char::from)
            .collect::<String>();
        Self(code)
    }
}

impl fmt::Display for AppraisalCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for AppraisalCode {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
