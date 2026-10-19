mod asteroid_utils;
mod compression;
mod fetch;
mod fetch_bulk;
mod fetch_bulk_reprocessing;
mod fetch_category;
mod fetch_group;
mod fetch_reprocessing;
mod gas_utils;
mod list;
mod mineral_utils;
mod parse;

pub mod error;

use std::collections::HashMap;

use starfoundry_gateway::ApiClient;
use starfoundry_lib_types::{CategoryId, GroupId, TypeId};
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

pub use self::asteroid_utils::*;
pub use self::compression::*;
pub(crate) use self::fetch::*;
pub(crate) use self::fetch_bulk::*;
pub(crate) use self::fetch_category::*;
pub(crate) use self::fetch_group::*;
pub use self::gas_utils::*;
pub(crate) use self::list::*;
pub use self::mineral_utils::*;
pub use self::parse::load_items;
pub(crate) use self::parse::*;
use crate::Result;
use crate::asset::{
    Category, Group, Item, ListItemFilter, ParseResult, Reprocessing
};
use crate::state::AppState;

/// Exposes all routes that are under `/items`
pub fn routes() -> OpenApiRouter<AppState> {
    let fetch = OpenApiRouter::new().routes(routes!(fetch::api));

    let fetch_bulk = OpenApiRouter::new().routes(routes!(fetch_bulk::api));

    let fetch_category =
        OpenApiRouter::new().routes(routes!(fetch_category::api));

    let fetch_group = OpenApiRouter::new().routes(routes!(fetch_group::api));

    let fetch_reprocessing =
        OpenApiRouter::new().routes(routes!(fetch_reprocessing::api));
    let fetch_reprocessing_bulk =
        OpenApiRouter::new().routes(routes!(fetch_bulk_reprocessing::api));

    let list = OpenApiRouter::new().routes(routes!(list::api));

    let parse = OpenApiRouter::new().routes(routes!(parse::api));

    OpenApiRouter::new()
        .merge(fetch)
        .merge(fetch_bulk)
        .merge(fetch_category)
        .merge(fetch_group)
        .merge(fetch_reprocessing)
        .merge(fetch_reprocessing_bulk)
        .merge(list)
        .merge(parse)
}

pub trait EveGatewayApiClientItem: ApiClient {
    #[allow(async_fn_in_trait)]
    async fn fetch_item(
        &self,
        type_id: TypeId,
    ) -> Result<Option<Item>> {
        self.fetch(&format!("items/{}", *type_id), &())
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn fetch_item_bulk(
        &self,
        type_ids: Vec<TypeId>,
    ) -> Result<Vec<Item>> {
        let mut type_ids = type_ids;
        type_ids.sort();
        type_ids.dedup();

        self.post("items", type_ids)
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn fetch_reprocessing(
        &self,
        type_id: TypeId,
    ) -> Result<Vec<Reprocessing>> {
        self.fetch(&format!("items/{}/reprocessing", *type_id), &())
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn fetch_bulk_reprocessing(
        &self,
        type_ids: Vec<TypeId>,
    ) -> Result<HashMap<TypeId, Vec<Reprocessing>>> {
        let mut type_ids = type_ids;
        type_ids.sort();
        type_ids.dedup();

        self.post("items/reprocessing", type_ids)
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn fetch_category(
        &self,
        category_id: CategoryId,
    ) -> Result<Option<Category>> {
        self.fetch(&format!("items/category/{}", *category_id), &())
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn fetch_group(
        &self,
        group_id: GroupId,
    ) -> Result<Option<Group>> {
        self.fetch(&format!("items/group/{}", *group_id), &())
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
        self.post("items/parse", items)
            .await
            .map_err(Into::into)
    }
}
