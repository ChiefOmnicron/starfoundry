mod standing;

use starfoundry_lib_gateway::ApiClient;

use crate::{ApiClientExtended, Result};
use crate::standing::standing::Standing;

pub trait EveGatewayApiClientStanding: ApiClient + ApiClientExtended {
    #[allow(async_fn_in_trait)]
    async fn list_alliance_standings(
        &self,
    ) -> Result<Vec<Standing>> {
        self
            .fetch_page(&format!("proxy/auth/alliances/contacts"))
            .await
    }

    #[allow(async_fn_in_trait)]
    async fn list_character_standings(
        &self,
    ) -> Result<Vec<Standing>> {
        self
            .fetch_page(&format!("proxy/auth/characters/contacts"))
            .await
    }

    #[allow(async_fn_in_trait)]
    async fn list_corporation_standings(
        &self,
    ) -> Result<Vec<Standing>> {
        self
            .fetch_page(&format!("proxy/auth/corporations/contacts"))
            .await
    }
}
