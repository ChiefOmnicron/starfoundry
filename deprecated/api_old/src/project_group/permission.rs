mod assert_exists;
mod assert_owner;
mod assert_read;
mod assert_write;

pub use self::assert_exists::*;
pub use self::assert_owner::*;
pub use self::assert_read::*;
pub use self::assert_write::*;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use std::ops::Deref;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ProjectGroupPermission(i32);

impl ProjectGroupPermission {
    pub fn new(permission: i32) -> Self {
        Self(permission)
    }

    pub fn is_owner(&self) -> bool {
        self.0 & 1 == 1
    }
}

impl Deref for ProjectGroupPermission {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub enum ProjectGroupPermissionCode {
    Owner,
    ReadGroup,
    WriteProject,
    WriteStructure,
    WriteDefault,
    WriteMember,
    WriteGroup,
}

impl Deref for ProjectGroupPermissionCode {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Owner             => &1i32,
            Self::ReadGroup         => &2i32,
            Self::WriteProject      => &4i32,
            Self::WriteStructure    => &8i32,
            Self::WriteDefault      => &16i32,
            Self::WriteMember       => &32i32,
            Self::WriteGroup        => &64i32,
        }
    }
}
