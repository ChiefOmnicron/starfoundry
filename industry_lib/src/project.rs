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

use starfoundry_lib_gateway::ApiClient;

use crate::{MarketUuid, ProjectJobUuid, ProjectUuid, Result};

pub trait IndustryApiClientProject: ApiClient {
    #[allow(async_fn_in_trait)]
    async fn project_add_excess_entry(
        &self,
        project_id: &ProjectUuid,
        request:    &AddExcessEntryRequest,
    ) -> Result<()> {

        self
            .post(
                format!("projects/{project_id}/excess"),
                request,
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn project_add_market_entry(
        &self,
        project_id: &ProjectUuid,
        request:    &AddMarketEntryRequest,
    ) -> Result<()> {

        self
            .post(
                format!("projects/{project_id}/market"),
                request,
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn project_add_job_entry(
        &self,
        project_id: &ProjectUuid,
        request:    &AddJobEntryRequest,
    ) -> Result<()> {

        self
            .post(
                format!("projects/{project_id}/market"),
                request,
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn project_create(
        &self,
        request: &CreateProject,
    ) -> Result<CreateProjectResponse> {

        self
            .post(
                "projects",
                request,
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn project_delete(
        &self,
        project_id: &ProjectUuid,
    ) -> Result<()> {
        self
            .delete(
                format!("projects/{project_id}"),
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn project_delete_market_entry(
        &self,
        project_id: &ProjectUuid,
        market_id:  &MarketUuid,
    ) -> Result<()> {
        self
            .delete(
                format!("projects/{project_id}/market/{market_id}"),
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn project_fetch(
        &self,
        project_id: &ProjectUuid,
    ) -> Result<Option<Project>> {
        self
            .fetch(
                format!("projects/{project_id}"),
                &(),
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn project_fetch_cost(
        &self,
        project_id: &ProjectUuid,
    ) -> Result<Option<ProjectCost>> {
        self
            .fetch(
                format!("projects/{project_id}/cost"),
                &(),
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn project_fetch_time_left(
        &self,
        project_id: &ProjectUuid,
    ) -> Result<Option<ProjectTimeLeft>> {
        self
            .fetch(
                format!("projects/{project_id}/time-left"),
                &(),
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn project_list(
        &self,
        filter: &ProjectFilter,
    ) -> Result<Vec<ProjectMinimal>> {

        self
            .fetch(
                "projects",
                filter,
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn project_list_excess(
        &self,
        project_id: &ProjectUuid,
        filter:     &ProjectJobFilter,
    ) -> Result<Vec<ProjectJob>> {

        self
            .fetch(
                format!("projects/{project_id}/excess"),
                filter,
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn project_list_jobs(
        &self,
        project_id: &ProjectUuid,
        filter:     &ProjectJobFilter,
    ) -> Result<Vec<ProjectJob>> {

        self
            .fetch(
                format!("projects/{project_id}/jobs"),
                filter,
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn project_list_all_jobs(
        &self,
    ) -> Result<Vec<ProjectJobAllGroup>> {
        self
            .fetch(
                format!("projects/jobs"),
                &(),
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn project_list_market(
        &self,
        project_id: &ProjectUuid,
    ) -> Result<Vec<ProjectMarket>> {
        self
            .fetch(
                format!("projects/{project_id}/market"),
                &(),
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn project_list_market_buy(
        &self,
        project_id: &ProjectUuid,
    ) -> Result<Vec<ProjectMarketBuy>> {
        self
            .fetch(
                format!("projects/{project_id}/market/buy"),
                &(),
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn project_list_misc(
        &self,
        project_id: &ProjectUuid,
    ) -> Result<Vec<ProjectMisc>> {
        self
            .fetch(
                format!("projects/{project_id}/misc"),
                &(),
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn project_list_stock(
        &self,
        project_id: &ProjectUuid,
    ) -> Result<Vec<ProjectStock>> {
        self
            .fetch(
                format!("projects/{project_id}/stock"),
                &(),
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn project_update(
        &self,
        project_id: &ProjectUuid,
        request:    &UpdateProject,
    ) -> Result<()> {
        self
            .put(
                format!("projects/{project_id}"),
                request,
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn project_update_job(
        &self,
        project_id: &ProjectUuid,
        job_id:     &ProjectJobUuid,
        request:    &UpdateMarketBulk,
    ) -> Result<()> {
        self
            .put(
                format!("projects/{project_id}/jobs/{job_id}"),
                request,
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn project_update_market_bulk(
        &self,
        project_id: &ProjectUuid,
        request:    &UpdateMarketBulk,
    ) -> Result<()> {
        self
            .put(
                format!("projects/{project_id}/market"),
                request,
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn project_update_market_entry(
        &self,
        project_id: &ProjectUuid,
        market_id:  &MarketUuid,
        request:    &UpdateMarketEntry,
    ) -> Result<()> {
        self
            .put(
                format!("projects/{project_id}/market/{market_id}"),
                request,
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn project_update_misc(
        &self,
        project_id: &ProjectUuid,
        request:    &UpdateMisc,
    ) -> Result<()> {
        self
            .put(
                format!("projects/{project_id}/misc"),
                request,
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn project_check_materials(
        &self,
        request: &CheckMaterialsRequest,
    ) -> Result<CheckMaterialsResponse> {
        self
            .post(
                "projects/check",
                request,
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn project_split_job_check(
        &self,
        project_id: &ProjectUuid,
        request:    &SplitJobRequest,
    ) -> Result<SplitJobResponse> {
        self
            .put(
                format!("projects/{project_id}/split-job/check"),
                request,
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn project_update_orderer(
        &self,
        project_id: &ProjectUuid,
        request:    String,
    ) -> Result<()> {
        self
            .put(
                format!("projects/{project_id}/orderer"),
                request,
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn project_update_notes(
        &self,
        project_id: &ProjectUuid,
        request:    String,
    ) -> Result<()> {
        self
            .put(
                format!("projects/{project_id}/notes"),
                request,
            )
            .await
            .map_err(Into::into)
    }
}
