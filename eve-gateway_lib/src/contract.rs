mod public_item;
mod public;

pub use self::public_item::*;
pub use self::public::*;

use starfoundry_lib_gateway::ApiClient;
use starfoundry_lib_types::{ContractId, RegionId};

use crate::error::Result;

pub trait EveGatewayApiClientContract: ApiClient {
    #[allow(async_fn_in_trait)]
    async fn fetch_public_contracts(
        &self,
        region_id: RegionId,
    ) -> Result<Vec<PublicContract>> {
        self
            .fetch(&format!("contracts/public/region/{}", *region_id), &())
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn fetch_public_contract_items(
        &self,
        contract_id: ContractId,
    ) -> Result<Vec<PublicContractItem>> {
        self
            .fetch(&format!("contracts/public/{}/items", *contract_id), &())
            .await
            .map_err(Into::into)
    }
}
