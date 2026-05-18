mod add;
mod check;
mod create;
mod fetch;
mod list;
mod update;
mod status;

pub use self::add::*;
pub use self::create::*;
pub use self::check::*;
pub use self::fetch::*;
pub use self::list::*;
pub use self::update::*;
pub use self::status::*;

use axum::http::{HeaderMap, HeaderValue};
use reqwest::header::HOST;
use starfoundry_lib_gateway::{ApiClient, HEADER_CHARACTER_ID};
use starfoundry_lib_types::CharacterId;

use crate::{MarketUuid, ProjectJobUuid, ProjectUuid, Result};

pub trait IndustryApiClientProject: ApiClient {
    #[allow(async_fn_in_trait)]
    async fn add_excess_entry(
        &self,
        source:         &String,
        character_id:   &CharacterId,
        project_id:     &ProjectUuid,
        request:        &AddExcessEntryRequest,
    ) -> Result<()> {
        let mut headers = HeaderMap::new();
        headers.insert(HOST, HeaderValue::from_str(&source).unwrap_or(HeaderValue::from_static("invalid.header")));
        headers.insert(HEADER_CHARACTER_ID, (**character_id).into());

        self
            .post_auth(
                format!("projects/{project_id}/excess"),
                request,
                headers,
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn add_market_entry(
        &self,
        source:         &String,
        character_id:   &CharacterId,
        project_id:     &ProjectUuid,
        request:        &AddMarketEntryRequest,
    ) -> Result<()> {
        let mut headers = HeaderMap::new();
        headers.insert(HOST, HeaderValue::from_str(&source).unwrap_or(HeaderValue::from_static("invalid.header")));
        headers.insert(HEADER_CHARACTER_ID, (**character_id).into());

        self
            .post_auth(
                format!("projects/{project_id}/market"),
                request,
                headers,
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn add_job_entry(
        &self,
        source:         &String,
        character_id:   &CharacterId,
        project_id:     &ProjectUuid,
        request:        &AddJobEntryRequest,
    ) -> Result<()> {
        let mut headers = HeaderMap::new();
        headers.insert(HOST, HeaderValue::from_str(&source).unwrap_or(HeaderValue::from_static("invalid.header")));
        headers.insert(HEADER_CHARACTER_ID, (**character_id).into());

        self
            .post_auth(
                format!("projects/{project_id}/market"),
                request,
                headers,
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn create(
        &self,
        source:         &String,
        character_id:   &CharacterId,
        request:        &CreateProject,
    ) -> Result<CreateProjectResponse> {
        let mut headers = HeaderMap::new();
        headers.insert(HOST, HeaderValue::from_str(&source).unwrap_or(HeaderValue::from_static("invalid.header")));
        headers.insert(HEADER_CHARACTER_ID, (**character_id).into());

        self
            .post_auth(
                "projects",
                request,
                headers,
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn delete(
        &self,
        source:         &String,
        character_id:   &CharacterId,
        project_id:     &ProjectUuid,
    ) -> Result<()> {
        let mut headers = HeaderMap::new();
        headers.insert(HOST, HeaderValue::from_str(&source).unwrap_or(HeaderValue::from_static("invalid.header")));
        headers.insert(HEADER_CHARACTER_ID, (**character_id).into());

        self
            .delete_auth(
                format!("projects/{project_id}"),
                headers,
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn delete_market_entry(
        &self,
        source:         &String,
        character_id:   &CharacterId,
        project_id:     &ProjectUuid,
        market_id:      &MarketUuid,
    ) -> Result<()> {
        let mut headers = HeaderMap::new();
        headers.insert(HOST, HeaderValue::from_str(&source).unwrap_or(HeaderValue::from_static("invalid.header")));
        headers.insert(HEADER_CHARACTER_ID, (**character_id).into());

        self
            .delete_auth(
                format!("projects/{project_id}/market/{market_id}"),
                headers,
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn fetch(
        &self,
        source:         &String,
        character_id:   &CharacterId,
        project_id:     &ProjectUuid,
    ) -> Result<Project> {
        let mut headers = HeaderMap::new();
        headers.insert(HOST, HeaderValue::from_str(&source).unwrap_or(HeaderValue::from_static("invalid.header")));
        headers.insert(HEADER_CHARACTER_ID, (**character_id).into());

        self
            .fetch_auth(
                format!("projects/{project_id}"),
                &(),
                headers,
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn fetch_cost(
        &self,
        source:         &String,
        character_id:   &CharacterId,
        project_id:     &ProjectUuid,
    ) -> Result<ProjectCost> {
        let mut headers = HeaderMap::new();
        headers.insert(HOST, HeaderValue::from_str(&source).unwrap_or(HeaderValue::from_static("invalid.header")));
        headers.insert(HEADER_CHARACTER_ID, (**character_id).into());

        self
            .fetch_auth(
                format!("projects/{project_id}/cost"),
                &(),
                headers,
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn fetch_time_left(
        &self,
        source:         &String,
        character_id:   &CharacterId,
        project_id:     &ProjectUuid,
    ) -> Result<ProjectTimeLeft> {
        let mut headers = HeaderMap::new();
        headers.insert(HOST, HeaderValue::from_str(&source).unwrap_or(HeaderValue::from_static("invalid.header")));
        headers.insert(HEADER_CHARACTER_ID, (**character_id).into());

        self
            .fetch_auth(
                format!("projects/{project_id}/time-left"),
                &(),
                headers,
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn list(
        &self,
        source:         &String,
        character_id:   &CharacterId,
        filter:         &ProjectFilter,
    ) -> Result<Vec<ProjectMinimal>> {
        let mut headers = HeaderMap::new();
        headers.insert(HOST, HeaderValue::from_str(&source).unwrap_or(HeaderValue::from_static("invalid.header")));
        headers.insert(HEADER_CHARACTER_ID, (**character_id).into());

        self
            .fetch_auth(
                "projects",
                filter,
                headers,
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn list_excess(
        &self,
        source:         &String,
        character_id:   &CharacterId,
        project_id:     &ProjectUuid,
        filter:         &ProjectJobFilter,
    ) -> Result<Vec<ProjectJob>> {
        let mut headers = HeaderMap::new();
        headers.insert(HOST, HeaderValue::from_str(&source).unwrap_or(HeaderValue::from_static("invalid.header")));
        headers.insert(HEADER_CHARACTER_ID, (**character_id).into());

        self
            .fetch_auth(
                format!("projects/{project_id}/excess"),
                filter,
                headers,
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn list_jobs(
        &self,
        source:         &String,
        character_id:   &CharacterId,
        project_id:     &ProjectUuid,
        filter:         &ProjectJobFilter,
    ) -> Result<Vec<ProjectJob>> {
        let mut headers = HeaderMap::new();
        headers.insert(HOST, HeaderValue::from_str(&source).unwrap_or(HeaderValue::from_static("invalid.header")));
        headers.insert(HEADER_CHARACTER_ID, (**character_id).into());

        self
            .fetch_auth(
                format!("projects/{project_id}/jobs"),
                filter,
                headers,
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn list_all_jobs(
        &self,
        source:         &String,
        character_id:   &CharacterId,
    ) -> Result<Vec<ProjectJobAllGroup>> {
        let mut headers = HeaderMap::new();
        headers.insert(HOST, HeaderValue::from_str(&source).unwrap_or(HeaderValue::from_static("invalid.header")));
        headers.insert(HEADER_CHARACTER_ID, (**character_id).into());

        self
            .fetch_auth(
                format!("projects/jobs"),
                &(),
                headers,
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn list_market(
        &self,
        source:         &String,
        character_id:   &CharacterId,
        project_id:     &ProjectUuid,
    ) -> Result<Vec<ProjectMarket>> {
        let mut headers = HeaderMap::new();
        headers.insert(HOST, HeaderValue::from_str(&source).unwrap_or(HeaderValue::from_static("invalid.header")));
        headers.insert(HEADER_CHARACTER_ID, (**character_id).into());

        self
            .fetch_auth(
                format!("projects/{project_id}/market"),
                &(),
                headers,
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn list_market_buy(
        &self,
        source:         &String,
        character_id:   &CharacterId,
        project_id:     &ProjectUuid,
    ) -> Result<Vec<ProjectMarketBuy>> {
        let mut headers = HeaderMap::new();
        headers.insert(HOST, HeaderValue::from_str(&source).unwrap_or(HeaderValue::from_static("invalid.header")));
        headers.insert(HEADER_CHARACTER_ID, (**character_id).into());

        self
            .fetch_auth(
                format!("projects/{project_id}/market/buy"),
                &(),
                headers,
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn list_misc(
        &self,
        source:         &String,
        character_id:   &CharacterId,
        project_id:     &ProjectUuid,
    ) -> Result<Vec<ProjectMisc>> {
        let mut headers = HeaderMap::new();
        headers.insert(HOST, HeaderValue::from_str(&source).unwrap_or(HeaderValue::from_static("invalid.header")));
        headers.insert(HEADER_CHARACTER_ID, (**character_id).into());

        self
            .fetch_auth(
                format!("projects/{project_id}/misc"),
                &(),
                headers,
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn list_stock(
        &self,
        source:         &String,
        character_id:   &CharacterId,
        project_id:     &ProjectUuid,
    ) -> Result<Vec<ProjectStock>> {
        let mut headers = HeaderMap::new();
        headers.insert(HOST, HeaderValue::from_str(&source).unwrap_or(HeaderValue::from_static("invalid.header")));
        headers.insert(HEADER_CHARACTER_ID, (**character_id).into());

        self
            .fetch_auth(
                format!("projects/{project_id}/stock"),
                &(),
                headers,
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn update(
        &self,
        source:         &String,
        character_id:   &CharacterId,
        project_id:     &ProjectUuid,
        request:        &UpdateProject,
    ) -> Result<()> {
        let mut headers = HeaderMap::new();
        headers.insert(HOST, HeaderValue::from_str(&source).unwrap_or(HeaderValue::from_static("invalid.header")));
        headers.insert(HEADER_CHARACTER_ID, (**character_id).into());

        self
            .put_auth(
                format!("projects/{project_id}"),
                request,
                headers,
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn update_job(
        &self,
        source:         &String,
        character_id:   &CharacterId,
        project_id:     &ProjectUuid,
        job_id:         &ProjectJobUuid,
        request:        &UpdateMarketBulk,
    ) -> Result<()> {
        let mut headers = HeaderMap::new();
        headers.insert(HOST, HeaderValue::from_str(&source).unwrap_or(HeaderValue::from_static("invalid.header")));
        headers.insert(HEADER_CHARACTER_ID, (**character_id).into());

        self
            .put_auth(
                format!("projects/{project_id}/jobs/{job_id}"),
                request,
                headers,
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn update_market_bulk(
        &self,
        source:         &String,
        character_id:   &CharacterId,
        project_id:     &ProjectUuid,
        request:        &UpdateMarketBulk,
    ) -> Result<()> {
        let mut headers = HeaderMap::new();
        headers.insert(HOST, HeaderValue::from_str(&source).unwrap_or(HeaderValue::from_static("invalid.header")));
        headers.insert(HEADER_CHARACTER_ID, (**character_id).into());

        self
            .put_auth(
                format!("projects/{project_id}/market"),
                request,
                headers,
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn update_market_entry(
        &self,
        source:         &String,
        character_id:   &CharacterId,
        project_id:     &ProjectUuid,
        market_id:      &MarketUuid,
        request:        &UpdateMarketEntry,
    ) -> Result<()> {
        let mut headers = HeaderMap::new();
        headers.insert(HOST, HeaderValue::from_str(&source).unwrap_or(HeaderValue::from_static("invalid.header")));
        headers.insert(HEADER_CHARACTER_ID, (**character_id).into());

        self
            .put_auth(
                format!("projects/{project_id}/market/{market_id}"),
                request,
                headers,
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn update_misc(
        &self,
        source:         &String,
        character_id:   &CharacterId,
        project_id:     &ProjectUuid,
        request:        &UpdateMisc,
    ) -> Result<()> {
        let mut headers = HeaderMap::new();
        headers.insert(HOST, HeaderValue::from_str(&source).unwrap_or(HeaderValue::from_static("invalid.header")));
        headers.insert(HEADER_CHARACTER_ID, (**character_id).into());

        self
            .put_auth(
                format!("projects/{project_id}/misc"),
                request,
                headers,
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn check_materials(
        &self,
        source:         &String,
        character_id:   &CharacterId,
        request:        &CheckMaterialsRequest,
    ) -> Result<CheckMaterialsResponse> {
        let mut headers = HeaderMap::new();
        headers.insert(HOST, HeaderValue::from_str(&source).unwrap_or(HeaderValue::from_static("invalid.header")));
        headers.insert(HEADER_CHARACTER_ID, (**character_id).into());

        self
            .post_auth(
                "projects/check",
                request,
                headers,
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn split_job_check(
        &self,
        source:         &String,
        character_id:   &CharacterId,
        project_id:     &ProjectUuid,
        request:        &SplitJobRequest,
    ) -> Result<SplitJobResponse> {
        let mut headers = HeaderMap::new();
        headers.insert(HOST, HeaderValue::from_str(&source).unwrap_or(HeaderValue::from_static("invalid.header")));
        headers.insert(HEADER_CHARACTER_ID, (**character_id).into());

        self
            .put_auth(
                format!("projects/{project_id}/split-job/check"),
                request,
                headers,
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn update_orderer(
        &self,
        source:         &String,
        character_id:   &CharacterId,
        project_id:     &ProjectUuid,
        request:        String,
    ) -> Result<()> {
        let mut headers = HeaderMap::new();
        headers.insert(HOST, HeaderValue::from_str(&source).unwrap_or(HeaderValue::from_static("invalid.header")));
        headers.insert(HEADER_CHARACTER_ID, (**character_id).into());

        self
            .put_auth(
                format!("projects/{project_id}/orderer"),
                request,
                headers,
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn update_notes(
        &self,
        source:         &String,
        character_id:   &CharacterId,
        project_id:     &ProjectUuid,
        request:        String,
    ) -> Result<()> {
        let mut headers = HeaderMap::new();
        headers.insert(HOST, HeaderValue::from_str(&source).unwrap_or(HeaderValue::from_static("invalid.header")));
        headers.insert(HEADER_CHARACTER_ID, (**character_id).into());

        self
            .put_auth(
                format!("projects/{project_id}/notes"),
                request,
                headers,
            )
            .await
            .map_err(Into::into)
    }
}
