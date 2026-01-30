use serde::{Deserialize, Serialize};
use starfoundry_lib_gateway::ApiClient;
use starfoundry_lib_types::{CharacterId, TypeId};
use std::collections::HashMap;
use std::fmt;
use utoipa::IntoParams;

use crate::{Result, Structure};

pub trait IndustryApiClientInternal: ApiClient {
    #[allow(async_fn_in_trait)]
    async fn list_structures(
        &self,
        filter: InternalStructureFilter,
    ) -> Result<HashMap<CharacterId, Vec<Structure>>> {
        self
            .fetch(&format!("internal/structures"), &filter)
            .await
            .map_err(Into::into)
    }
}

#[derive(Debug, Default, Deserialize, Serialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct InternalStructureFilter {
    /// [TypeId] of a structure service
    #[serde(default)]
    #[param(
        example = json!("35892"),
        required = false,
    )]
    pub service_id: Option<TypeId>,
}

impl fmt::Display for InternalStructureFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self, f)
    }
}
