mod assert_exists;
mod assert_owner;
mod assert_read;
mod assert_write;

pub use self::assert_exists::*;
pub use self::assert_owner::*;
pub use self::assert_read::*;
pub use self::assert_write::*;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use utoipa::ToSchema;
use std::ops::Deref;
use serde::ser::SerializeSeq;
use serde::de::Visitor;

const BIT_OWNER:            i32 = 1i32;
const BIT_READ:             i32 = 2i32;
const BIT_WRITE_PROJECT:    i32 = 4i32;
const BIT_WRITE_STRUCTURE:  i32 = 8i32;
const BIT_WRITE_DEFAULT:    i32 = 16i32;
const BIT_WRITE_MEMBER:     i32 = 32i32;
const BIT_WRITE_GROUP:      i32 = 64i32;

#[derive(Debug, ToSchema)]
pub struct ProjectGroupPermission(i32);

impl ProjectGroupPermission {
    pub fn new(permission: i32) -> Self {
        Self(permission)
    }

    pub fn add(&mut self, permission: ProjectGroupPermissionCode) -> &mut Self {
        self.0 += *permission;
        self
    }

    pub fn as_permissions(
        &self,
    ) -> Vec<ProjectGroupPermissionCode> {
        let mut permissions = Vec::new();

        if self.0 & BIT_OWNER == BIT_OWNER {
            permissions.push(ProjectGroupPermissionCode::Owner)
        }
        if self.0 & BIT_READ == BIT_READ {
            permissions.push(ProjectGroupPermissionCode::Read)
        }
        if self.0 & BIT_WRITE_PROJECT == BIT_WRITE_PROJECT {
            permissions.push(ProjectGroupPermissionCode::WriteProject)
        }
        if self.0 & BIT_WRITE_STRUCTURE == BIT_WRITE_STRUCTURE {
            permissions.push(ProjectGroupPermissionCode::WriteStructure)
        }
        if self.0 & BIT_WRITE_DEFAULT == BIT_WRITE_DEFAULT {
            permissions.push(ProjectGroupPermissionCode::WriteDefault)
        }
        if self.0 & BIT_WRITE_MEMBER == BIT_WRITE_MEMBER {
            permissions.push(ProjectGroupPermissionCode::WriteMember)
        }
        if self.0 & BIT_WRITE_GROUP == BIT_WRITE_GROUP {
            permissions.push(ProjectGroupPermissionCode::WriteGroup)
        }

        permissions
    }
}

impl Deref for ProjectGroupPermission {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct PermissionVisitor;

impl<'de> Visitor<'de> for PermissionVisitor {
    type Value = ProjectGroupPermission;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Expected array of strings")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::SeqAccess<'de>, {

        let mut permission = ProjectGroupPermission::new(0);

        while let Ok(Some(x)) = seq.next_element::<ProjectGroupPermissionCode>() {
            permission.add(x);
        }

        Ok(permission)
    }
}

impl<'de> Deserialize<'de> for ProjectGroupPermission {
    fn deserialize<D>(deserializer: D) -> Result<ProjectGroupPermission, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(PermissionVisitor)
    }
}

impl Serialize for ProjectGroupPermission {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let permissions = self.as_permissions();

        let mut seq = serializer.serialize_seq(Some(permissions.len()))?;
        for permission in permissions {
            seq.serialize_element(&permission)?;
        }
        seq.end()
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProjectGroupPermissionCode {
    Owner,
    Read,
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
            Self::Owner          => &BIT_OWNER,
            Self::Read           => &BIT_READ,
            Self::WriteProject   => &BIT_WRITE_PROJECT,
            Self::WriteStructure => &BIT_WRITE_STRUCTURE,
            Self::WriteDefault   => &BIT_WRITE_DEFAULT,
            Self::WriteMember    => &BIT_WRITE_MEMBER,
            Self::WriteGroup     => &BIT_WRITE_GROUP,
        }
    }
}
