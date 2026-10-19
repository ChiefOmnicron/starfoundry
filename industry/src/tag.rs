pub mod create;
pub mod delete;
pub mod fetch;
pub mod list;
pub mod trigger;
pub mod update;

pub mod error;

use serde::{Deserialize, Serialize};
use starfoundry_gateway::ApiClient;
use utoipa::ToSchema;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::tag::create::{CreateTag, CreateTagResponse};
use crate::tag::error::TagError;
use crate::tag::fetch::Tag;
use crate::tag::update::UpdateTag;
use crate::{AppState, Result, TagUuid};

pub fn routes() -> OpenApiRouter<AppState> {
    let list = OpenApiRouter::new().routes(routes!(list::api));

    let fetch = OpenApiRouter::new().routes(routes!(fetch::api));

    let create = OpenApiRouter::new().routes(routes!(create::api));

    let update = OpenApiRouter::new().routes(routes!(update::api));

    let delete = OpenApiRouter::new().routes(routes!(delete::api));

    let trigger = OpenApiRouter::new().routes(routes!(trigger::api));

    OpenApiRouter::new()
        .merge(list)
        .merge(fetch)
        .merge(create)
        .merge(update)
        .merge(delete)
        .merge(trigger)
}

pub trait IndustryApiClientTag: ApiClient {
    #[allow(async_fn_in_trait)]
    async fn tag_create(
        &self,
        request: &CreateTag,
    ) -> Result<CreateTagResponse> {
        self.post("tags", request)
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn tag_delete(
        &self,
        tag_id: &TagUuid,
    ) -> Result<()> {
        self.delete(format!("tags/{tag_id}"))
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn tag_fetch(
        &self,
        tag_id: &TagUuid,
    ) -> Result<Option<Tag>> {
        self.fetch(format!("projects/{tag_id}"), &())
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn tag_list(&self) -> Result<Vec<Tag>> {
        self.fetch("tags", &())
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn tag_update(
        &self,
        tag_id: &TagUuid,
        request: &UpdateTag,
    ) -> Result<()> {
        self.put(format!("tags/{tag_id}"), request)
            .await
            .map_err(Into::into)
    }
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct TagAuto {
    pub select:  TagAutoSelect,
    pub compare: TagAutoCompare,
    pub value:   String,
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
            Self::Auto => "AUTO",
            Self::Manual => "MANUAL",
        }
    }
}

impl TryFrom<String> for TagType {
    type Error = TagError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_ref() {
            "AUTO" => Ok(Self::Auto),
            "MANUAL" => Ok(Self::Manual),
            _ => Err(Self::Error::EnumParseError(value, "TagType")),
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
            Self::ProjectName => "PROJECT_NAME",
            Self::ProjectOrderer => "PROJECT_ORDER",
            Self::ProjectStatus => "PROJECT_STATUS",
        }
    }
}

impl TryFrom<String> for TagAutoSelect {
    type Error = TagError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_ref() {
            "PROJECT_NAME" => Ok(Self::ProjectName),
            "PROJECT_ORDERER" => Ok(Self::ProjectOrderer),
            "PROJECT_STATUS" => Ok(Self::ProjectStatus),
            _ => Err(Self::Error::EnumParseError(value, "TagAutoSelect")),
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
            Self::Is => "IS",
            Self::IsNot => "IS_NOT",
            Self::Contains => "CONTAINS",
            Self::Pattern => "PATTERN",
        }
    }
}

impl TryFrom<String> for TagAutoCompare {
    type Error = TagError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_ref() {
            "IS" => Ok(Self::Is),
            "IS_NOT" => Ok(Self::IsNot),
            "CONTAINS" => Ok(Self::Contains),
            "PATTERN" => Ok(Self::Pattern),
            _ => Err(Self::Error::EnumParseError(value, "TagAutoCompare")),
        }
    }
}
