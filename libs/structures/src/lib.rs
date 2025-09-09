mod dynamic_group;
mod enums;
mod error;
mod group;
mod root;
mod service;

pub mod rig;

pub use self::dynamic_group::*;
pub use self::enums::*;
pub use self::error::*;
pub use self::group::*;
pub use self::rig::*;
pub use self::root::*;
pub use self::service::*;

use starfoundry_lib_types::starfoundry_uuid;
starfoundry_uuid!(StructureUuid, "StructureUuid");
starfoundry_uuid!(StructureGroupUuid, "StructureGroupUuid");
starfoundry_uuid!(StructureDynamicGroupUuid, "StructureDynamicGroupUuid");
