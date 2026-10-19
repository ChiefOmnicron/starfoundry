mod create;
mod delete;

pub mod error;

use serde::{Deserialize, Serialize};
use starfoundry_gateway::{ApiClient, Identity};
use starfoundry_lib_types::{FittingId, TypeId};
use utoipa::ToSchema;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::asset::LocationFlag;
use crate::fitting::error::Result;
use crate::state::AppState;

/// Exposes all routes that are under `/fitting`
pub fn routes() -> OpenApiRouter<AppState> {
    let create = OpenApiRouter::new().routes(routes!(create::api));

    let delete = OpenApiRouter::new().routes(routes!(delete::api));

    OpenApiRouter::new()
        .merge(create)
        .merge(delete)
}

pub trait EveGatewayApiClientFitting: ApiClient {
    #[allow(async_fn_in_trait)]
    async fn create_fit(
        &self,
        identity: Identity,
        data: EveFit,
    ) -> Result<EveFitResponse> {
        self.post(
            &format!("characters/{}/fittings", identity.character_id()),
            data,
        )
        .await
        .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn delete_fit(
        &self,
        identity: Identity,
        fitting_id: FittingId,
    ) -> Result<EveFitResponse> {
        self.delete(&format!(
            "characters/{}/fittings/{}",
            identity.character_id(),
            fitting_id,
        ))
        .await
        .map_err(Into::into)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
#[schema(
    example = json!({
        "name": "My cool fit",
        "description": "You should check it out",
        "items": [{
            "quantity": 1,
            "type_id": 34489,
            "flag": "LoSlot0"
        }],
        "ship_type_id": 20185
    })
)]
pub struct EveFit {
    pub name:         String,
    pub description:  String,
    pub items:        Vec<EveFitItem>,
    pub ship_type_id: TypeId,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
#[schema(
    example = json!({
        "quantity": 1,
        "type_id": 34489,
        "flag": "LoSlot0"
    })
)]
pub struct EveFitItem {
    pub quantity: i64,
    pub type_id:  TypeId,
    pub flag:     LocationFlag,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct EveFitResponse {
    fitting_id: FittingId,
}

impl Default for EveFitResponse {
    fn default() -> Self {
        Self {
            fitting_id: FittingId(0),
        }
    }
}
