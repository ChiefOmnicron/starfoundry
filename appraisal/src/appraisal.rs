mod compression;
mod create;
mod error;
mod fetch;
mod reprocessing;
mod settings;

pub use self::error::*;

use rand::distr::Alphanumeric;
use rand::RngExt;
use serde::{Deserialize, Serialize};
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::AppState;
use std::fmt;
use std::ops::Deref;
use utoipa::ToSchema;

pub fn routes(
    state: AppState,
) -> OpenApiRouter<AppState> {
    let create = OpenApiRouter::new()
        .routes(routes!(create::api));
    //let fetch = OpenApiRouter::new()
    //    .routes(routes!(fetch::api));

    //let compression = OpenApiRouter::new()
    //    .routes(routes!(compression::api));
    //let reprocessing = OpenApiRouter::new()
    //    .routes(routes!(reprocessing::api));

    //let settings = OpenApiRouter::new()
    //    .routes(routes!(settings::api));

    OpenApiRouter::new()
        .merge(create)
        //.merge(fetch)

        //.merge(compression)
        //.merge(reprocessing)

        //.merge(settings)
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
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
