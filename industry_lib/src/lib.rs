mod client;
mod error;
mod industry;
mod industry_hub;
mod structure;

use starfoundry_lib_types::starfoundry_uuid;

pub use self::client::*;
pub use self::error::*;
pub use self::industry::*;
pub use self::industry_hub::*;
pub use self::structure::*;

pub const ENV_INDUSTRY_API: &str = "STARFOUNDRY_INDUSTRY_API_URL";

starfoundry_uuid!(IndustryHubUuid, "IndustryHubUuid");
starfoundry_uuid!(MarketUuid, "MarketUuid");
starfoundry_uuid!(ProjectGroupUuid, "ProjectGroupUuid");
starfoundry_uuid!(ProjectJobUuid, "ProjectJobUuid");
starfoundry_uuid!(ProjectUuid, "ProjectUuid");
starfoundry_uuid!(SolutionUuid, "SolutionUuid");

