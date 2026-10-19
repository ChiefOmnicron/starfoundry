use serde::{Deserialize, Serialize};
use starfoundry_gateway::ApiClient;
use starfoundry_lib_types::ContactId;
use utoipa::ToSchema;

use crate::Result;
use crate::client::ApiClientExtended;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct Standing {
    pub contact_id:   ContactId,
    pub contact_type: String,
    pub standing:     f32,
}

pub trait EveGatewayApiClientStanding: ApiClient + ApiClientExtended {
    #[allow(async_fn_in_trait)]
    async fn list_alliance_standings(&self) -> Result<Vec<Standing>> {
        self.fetch_page("proxy/auth/alliances/contacts")
            .await
    }

    #[allow(async_fn_in_trait)]
    async fn list_character_standings(&self) -> Result<Vec<Standing>> {
        self.fetch_page("proxy/auth/characters/contacts")
            .await
    }

    #[allow(async_fn_in_trait)]
    async fn list_corporation_standings(&self) -> Result<Vec<Standing>> {
        self.fetch_page("proxy/auth/corporations/contacts")
            .await
    }
}
