use serde::{Deserialize, Serialize};
use starfoundry_lib_types::{CategoryId, GroupId, TypeId};
use utoipa::{IntoParams, ToSchema};
use starfoundry_lib_gateway::ApiClient;

use crate::Result;

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
#[schema(
    example = json!({
        "base_price": null,
        "category": {
            "category_id": 0,
            "name": "#System"
        },
        "group": {
            "group_id": 0,
            "category_id": 0,
            "name": "#System"
        },
        "meta_group_id": null,
        "name": "Ragnarok",
        "repackaged": 10000000,
        "type_id": 23773,
        "volume": 100000000
    })
)]
pub struct Item {
    pub type_id:    TypeId,
    pub category:   Category,
    pub group:      Group,
    pub volume:     f32,
    pub name:       String,

    pub meta_group: Option<GroupId>,
    pub repackaged: Option<i32>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
#[schema(
    example = json!({
        "category_id": 0,
        "name": "#System"
    })
)]
pub struct Category {
    pub category_id: CategoryId,
    pub name:        String,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
#[schema(
    example = json!({
        "group_id": 0,
        "category_id": 0,
        "name": "#System"
    })
)]
pub struct Group {
    pub group_id:    GroupId,
    pub category_id: CategoryId,
    pub name:        String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, ToSchema, IntoParams)]
#[schema(
    example = json!({
        "name": "Pyerite"
    })
)]
pub struct ListItemFilter {
    #[serde(default)]
    pub name: Option<String>,

    /// Only searches for items that are buildable
    #[serde(default)]
    pub buildable: Option<bool>,
    /// Only searches for blueprints
    #[serde(default)]
    pub blueprint: Option<bool>,

    #[serde(default)]
    pub categories: Option<Vec<CategoryId>>,
    #[serde(default)]
    pub groups:     Option<Vec<GroupId>>,
    /// only for `buildable = true`. Filters out blueprints that cannot be build
    /// with the given services
    #[serde(default)]
    pub services:   Option<Vec<TypeId>>,

    #[serde(default = "list_item_filter_limit_default")]
    pub limit:      Option<i64>,
}

fn list_item_filter_limit_default() -> Option<i64> {
    Some(20)
}

#[derive(Clone, Deserialize, Debug, Serialize, ToSchema)]
pub struct ParsedItem {
    pub item_name:           String,
    pub quantity:            i64,
    pub type_id:             TypeId,
    pub material_efficiency: Option<usize>,

    pub raw:                 Item,
}

#[derive(Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct ParseResult {
    pub items:   Vec<ParsedItem>,
    pub invalid: Vec<String>,
}

pub trait EveGatewayApiClientItem: ApiClient {
    #[allow(async_fn_in_trait)]
    async fn fetch_item(
        &self,
        type_id: TypeId,
    ) -> Result<Option<Item>> {
        self
            .fetch(&format!("items/{}", *type_id), &())
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn fetch_item_bulk(
        &self,
        type_ids: Vec<TypeId>,
    ) -> Result<Vec<Item>> {
        self
            .post("items", type_ids)
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn fetch_category(
        &self,
        category_id: CategoryId,
    ) -> Result<Option<Category>> {
        self
            .fetch(&format!("items/category/{}", *category_id), &())
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn fetch_group(
        &self,
        group_id: GroupId,
    ) -> Result<Option<Group>> {
        self
            .fetch(&format!("items/group/{}", *group_id), &())
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn list_items(
        &self,
        _filter: ListItemFilter,
    ) -> Result<Vec<Item>> {
        unimplemented!()
    }

    #[allow(async_fn_in_trait)]
    async fn parse_items(
        &self,
        items: String,
    ) -> Result<ParseResult> {
        self
            .post(&format!("items/parse"), items)
            .await
            .map_err(Into::into)
    }
}
