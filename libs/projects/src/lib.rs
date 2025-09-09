mod engine;
mod enums;
mod error;
mod excess;
mod finance;
mod group;
mod job_assignment;
mod job_detection;
mod job;
mod market;
mod misc;
mod product;
mod root;
mod service;
mod sort_by_job;
mod sort_by_market_group;
mod sort_by_product_group;
mod stock;
mod utils;

pub use self::engine::*;
pub use self::enums::*;
pub use self::error::*;
pub use self::excess::*;
pub use self::finance::*;
pub use self::group::*;
pub use self::job_assignment::*;
pub use self::job_detection::*;
pub use self::job::*;
pub use self::market::*;
pub use self::misc::*;
pub use self::product::*;
pub use self::root::*;
pub use self::service::*;
pub use self::stock::*;
pub use self::utils::*;

#[cfg(test)]
mod test_util;
#[cfg(test)]
pub use self::test_util::*;

use starfoundry_lib_types::starfoundry_uuid;
starfoundry_uuid!(ProjectUuid, "ProjectUuid");
starfoundry_uuid!(ProjectGroupUuid, "ProjectGroupUuid");
starfoundry_uuid!(ProjectJobAssignmentUuid, "ProjectJobAssignmentUuid");
starfoundry_uuid!(ProjectJobUuid, "ProjectJobUuid");
starfoundry_uuid!(ProjectMarketUuid, "ProjectMarketUuid");
starfoundry_uuid!(ProjectMiscUuid, "ProjectMiscUuid");
starfoundry_uuid!(ProjectProductUuid, "ProjectProductUuid");
