mod public_item;
mod public;

pub use self::public_item::*;
pub use self::public::*;

use starfoundry_lib_gateway::ApiClient;
use starfoundry_lib_types::{ContractId, RegionId};

use crate::error::Result;

pub trait EveGatewayApiClientContract: ApiClient {
    /// Lists all public contracts in the given region.
    /// 
    #[allow(async_fn_in_trait)]
    async fn list_public_contracts(
        &self,
        region_id: RegionId,
    ) -> Result<Vec<PublicContract>> {
        self
            .fetch(
                &format!("proxy/list/contracts/public/{}", region_id),
                &()
            )
            .await
            .map_err(Into::into)
    }

    /// Lists all items in the given contract.
    /// 
    #[allow(async_fn_in_trait)]
    async fn list_public_contract_items(
        &self,
        contract_id: ContractId,
    ) -> Result<Vec<PublicContractItem>> {
        self
            .fetch(
                &format!("proxy/list/contracts/public/items/{}", contract_id),
                &(),
            )
            .await
            .map_err(Into::into)
    }
}
