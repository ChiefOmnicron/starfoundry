mod client;
mod error;

pub mod industry_hub;
pub mod industry;
pub mod project;
pub mod project_group;
pub mod structure;
pub mod tag;

use starfoundry_lib_types::starfoundry_uuid;

pub use self::client::*;
pub use self::error::*;

// re-export from the market library
pub use starfoundry_lib_market::GasDecompressionEfficiency;
pub use starfoundry_lib_market::OreReprocessingEfficiency;

pub const ENV_INDUSTRY_API: &str = "STARFOUNDRY_INDUSTRY_API_URL";

starfoundry_uuid!(IndustryHubUuid, "IndustryHubUuid");
starfoundry_uuid!(MarketUuid, "MarketUuid");
starfoundry_uuid!(ProjectGroupUuid, "ProjectGroupUuid");
starfoundry_uuid!(ProjectJobUuid, "ProjectJobUuid");
starfoundry_uuid!(ProjectUuid, "ProjectUuid");
starfoundry_uuid!(SolutionUuid, "SolutionUuid");
starfoundry_uuid!(StructureUuid, "StructureUuid");
starfoundry_uuid!(TagUuid, "TagUuid");
