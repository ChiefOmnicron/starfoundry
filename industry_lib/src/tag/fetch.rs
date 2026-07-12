use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{Error, TagUuid};

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct Tag {
    pub id:         TagUuid,
    pub color:      String,
    pub content:    String,
    pub typ:        TagType,

    pub auto:       Vec<TagAuto>
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct TagAuto {
    pub select:     TagAutoSelect,
    pub compare:    TagAutoCompare,
    pub value:      String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TagType {
    Auto,
    Manual,
}

impl TagType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Auto      => "AUTO",
            Self::Manual    => "MANUAL",
        }
    }
}

impl TryFrom<String> for TagType {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_ref() {
            "AUTO"      => Ok(Self::Auto),
            "MANUAL"    => Ok(Self::Manual),
            _           => Err(Self::Error::EnumParseError(value, "TagType")),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, ToSchema, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TagAutoSelect {
    ProjectName,
    ProjectOrderer,
    ProjectStatus,
}

impl TagAutoSelect {
    pub fn as_str(&self) -> &str {
        match self {
            Self::ProjectName       => "PROJECT_NAME",
            Self::ProjectOrderer    => "PROJECT_ORDER",
            Self::ProjectStatus     => "PROJECT_STATUS",
        }
    }
}

impl TryFrom<String> for TagAutoSelect {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_ref() {
            "PROJECT_NAME"      => Ok(Self::ProjectName),
            "PROJECT_ORDERER"   => Ok(Self::ProjectOrderer),
            "PROJECT_STATUS"    => Ok(Self::ProjectStatus),
            _                   => Err(Self::Error::EnumParseError(value, "TagAutoSelect")),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, ToSchema, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TagAutoCompare {
    Is,
    IsNot,
    Contains,
    Pattern,
}

impl TagAutoCompare {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Is        => "IS",
            Self::IsNot     => "IS_NOT",
            Self::Contains  => "CONTAINS",
            Self::Pattern   => "PATTERN",
        }
    }
}

impl TryFrom<String> for TagAutoCompare {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_ref() {
            "IS"        => Ok(Self::Is),
            "IS_NOT"    => Ok(Self::IsNot),
            "CONTAINS"  => Ok(Self::Contains),
            "PATTERN"   => Ok(Self::Pattern),
            _           => Err(Self::Error::EnumParseError(value, "TagAutoCompare")),
        }
    }
}
