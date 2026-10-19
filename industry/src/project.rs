pub mod add_excess_entry;
pub mod add_job;
pub mod add_market;
pub mod check_resources;
pub mod create;
pub mod delete;
pub mod delete_job;
pub mod delete_market_entry;
pub mod fetch;
pub mod fetch_cost;
pub mod fetch_time_left;
pub mod initialize;
pub mod list;
pub mod list_all_jobs;
pub mod list_excess;
pub mod list_jobs;
pub mod list_market;
pub mod list_market_buy;
pub mod list_market_structures;
pub mod list_misc;
pub mod list_parent;
pub mod list_product;
pub mod list_stock;
pub mod list_sub;
pub mod list_tags;
pub mod permission;
pub mod split_job_check;
pub mod update;
pub mod update_job;
pub mod update_market_bulk;
pub mod update_market_entry;
pub mod update_misc;
pub mod update_notes;
pub mod update_orderer;
pub mod update_price;
pub mod update_sub_project;

pub mod error;

use axum::middleware;
use serde::{Deserialize, Serialize};
use starfoundry_gateway::ApiClient;
use utoipa::ToSchema;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::project::add_excess_entry::AddExcessEntryRequest;
use crate::project::add_job::AddJobEntryRequest;
use crate::project::add_market::AddMarketEntryRequest;
use crate::project::check_resources::{
    CheckMaterialsRequest, CheckMaterialsResponse
};
use crate::project::create::{CreateProject, CreateProjectResponse};
use crate::project::fetch::{
    Project, ProjectCost, ProjectMarket, ProjectMarketBuy, ProjectMisc, ProjectStock, ProjectTimeLeft
};
use crate::project::list::{
    ProjectFilter, ProjectJob, ProjectJobAllGroup, ProjectJobFilter, ProjectMinimal
};
use crate::project::permission::{assert_exists, assert_read};
use crate::project::split_job_check::{SplitJobRequest, SplitJobResponse};
use crate::project::update::{
    UpdateMarketBulk, UpdateMarketEntry, UpdateMisc, UpdateProject
};
use crate::{AppState, MarketUuid, ProjectJobUuid, ProjectUuid, Result};

// TODO: check write permission
pub fn routes(state: AppState) -> OpenApiRouter<AppState> {
    let create = OpenApiRouter::new().routes(routes!(create::api));

    let add_excess =
        OpenApiRouter::new().routes(routes!(add_excess_entry::api));
    let add_job = OpenApiRouter::new().routes(routes!(add_job::api));
    let add_market = OpenApiRouter::new().routes(routes!(add_market::api));

    let fetch = OpenApiRouter::new()
        .routes(routes!(fetch::api))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_read))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            assert_exists,
        ));
    let fetch_cost = OpenApiRouter::new()
        .routes(routes!(fetch_cost::api))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_read))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            assert_exists,
        ));
    let fetch_time_left = OpenApiRouter::new()
        .routes(routes!(fetch_time_left::api))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_read))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            assert_exists,
        ));

    let delete = OpenApiRouter::new()
        .routes(routes!(delete::api))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            assert_exists,
        ));
    let delete_market_entry =
        OpenApiRouter::new().routes(routes!(delete_market_entry::api));
    let delete_job = OpenApiRouter::new().routes(routes!(delete_job::api));

    let list = OpenApiRouter::new().routes(routes!(list::api));
    let list_jobs = OpenApiRouter::new()
        .routes(routes!(list_jobs::api))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_read))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            assert_exists,
        ));
    let list_all_jobs =
        OpenApiRouter::new().routes(routes!(list_all_jobs::api));
    let list_excess = OpenApiRouter::new()
        .routes(routes!(list_excess::api))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_read))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            assert_exists,
        ));
    let list_market = OpenApiRouter::new()
        .routes(routes!(list_market::api))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_read))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            assert_exists,
        ));
    let list_market_buy = OpenApiRouter::new()
        .routes(routes!(list_market_buy::api))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_read))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            assert_exists,
        ));
    let list_market_structures = OpenApiRouter::new()
        .routes(routes!(list_market_structures::api))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_read))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            assert_exists,
        ));
    let list_misc = OpenApiRouter::new()
        .routes(routes!(list_misc::api))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_read))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            assert_exists,
        ));

    let check_resources =
        OpenApiRouter::new().routes(routes!(check_resources::api));
    let split_job_check =
        OpenApiRouter::new().routes(routes!(split_job_check::api));

    let update = OpenApiRouter::new()
        .routes(routes!(update::api))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            assert_exists,
        ));
    let update_job = OpenApiRouter::new().routes(routes!(update_job::api));
    let update_market_bulk = OpenApiRouter::new()
        .routes(routes!(update_market_bulk::api))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            assert_exists,
        ));
    let update_market_entry =
        OpenApiRouter::new().routes(routes!(update_market_entry::api));
    let update_misc = OpenApiRouter::new()
        .routes(routes!(update_misc::api))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            assert_exists,
        ));
    let initialize = OpenApiRouter::new()
        .routes(routes!(initialize::api))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            assert_exists,
        ));

    let update_notes = OpenApiRouter::new()
        .routes(routes!(update_notes::api))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            assert_exists,
        ));
    let update_orderer = OpenApiRouter::new()
        .routes(routes!(update_orderer::api))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            assert_exists,
        ));
    let update_price = OpenApiRouter::new()
        .routes(routes!(update_price::api))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            assert_exists,
        ));
    let update_sub_project = OpenApiRouter::new()
        .routes(routes!(update_sub_project::api))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            assert_exists,
        ));

    OpenApiRouter::new()
        .merge(create)
        .merge(add_excess)
        .merge(add_job)
        .merge(add_market)
        .merge(fetch)
        .merge(fetch_cost)
        .merge(fetch_time_left)
        .merge(delete)
        .merge(delete_market_entry)
        .merge(delete_job)
        .merge(initialize)
        .merge(list)
        .merge(list_excess)
        .merge(list_jobs)
        .merge(list_all_jobs)
        .merge(list_market)
        .merge(list_market_buy)
        .merge(list_market_structures)
        .merge(list_misc)
        .merge(check_resources)
        .merge(split_job_check)
        .merge(update)
        .merge(update_job)
        .merge(update_market_bulk)
        .merge(update_market_entry)
        .merge(update_misc)
        .merge(update_orderer)
        .merge(update_notes)
        .merge(update_price)
        .merge(update_sub_project)
}

pub trait IndustryApiClientProject: ApiClient {
    #[allow(async_fn_in_trait)]
    async fn project_add_excess_entry(
        &self,
        project_id: &ProjectUuid,
        request: &AddExcessEntryRequest,
    ) -> Result<()> {
        self.post(format!("projects/{project_id}/excess"), request)
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn project_add_market_entry(
        &self,
        project_id: &ProjectUuid,
        request: &AddMarketEntryRequest,
    ) -> Result<()> {
        self.post(format!("projects/{project_id}/market"), request)
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn project_add_job_entry(
        &self,
        project_id: &ProjectUuid,
        request: &AddJobEntryRequest,
    ) -> Result<()> {
        self.post(format!("projects/{project_id}/market"), request)
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn project_create(
        &self,
        request: &CreateProject,
    ) -> Result<CreateProjectResponse> {
        self.post("projects", request)
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn project_delete(
        &self,
        project_id: &ProjectUuid,
    ) -> Result<()> {
        self.delete(format!("projects/{project_id}"))
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn project_delete_market_entry(
        &self,
        project_id: &ProjectUuid,
        market_id: &MarketUuid,
    ) -> Result<()> {
        self.delete(format!("projects/{project_id}/market/{market_id}"))
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn project_fetch(
        &self,
        project_id: &ProjectUuid,
    ) -> Result<Option<Project>> {
        self.fetch(format!("projects/{project_id}"), &())
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn project_fetch_cost(
        &self,
        project_id: &ProjectUuid,
    ) -> Result<Option<ProjectCost>> {
        self.fetch(format!("projects/{project_id}/cost"), &())
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn project_fetch_time_left(
        &self,
        project_id: &ProjectUuid,
    ) -> Result<Option<ProjectTimeLeft>> {
        self.fetch(format!("projects/{project_id}/time-left"), &())
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn project_list(
        &self,
        filter: &ProjectFilter,
    ) -> Result<Vec<ProjectMinimal>> {
        self.fetch("projects", filter)
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn project_list_excess(
        &self,
        project_id: &ProjectUuid,
        filter: &ProjectJobFilter,
    ) -> Result<Vec<ProjectJob>> {
        self.fetch(format!("projects/{project_id}/excess"), filter)
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn project_list_jobs(
        &self,
        project_id: &ProjectUuid,
        filter: &ProjectJobFilter,
    ) -> Result<Vec<ProjectJob>> {
        self.fetch(format!("projects/{project_id}/jobs"), filter)
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn project_list_all_jobs(&self) -> Result<Vec<ProjectJobAllGroup>> {
        self.fetch("projects/jobs", &())
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn project_list_market(
        &self,
        project_id: &ProjectUuid,
    ) -> Result<Vec<ProjectMarket>> {
        self.fetch(format!("projects/{project_id}/market"), &())
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn project_list_market_buy(
        &self,
        project_id: &ProjectUuid,
    ) -> Result<Vec<ProjectMarketBuy>> {
        self.fetch(format!("projects/{project_id}/market/buy"), &())
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn project_list_misc(
        &self,
        project_id: &ProjectUuid,
    ) -> Result<Vec<ProjectMisc>> {
        self.fetch(format!("projects/{project_id}/misc"), &())
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn project_list_stock(
        &self,
        project_id: &ProjectUuid,
    ) -> Result<Vec<ProjectStock>> {
        self.fetch(format!("projects/{project_id}/stock"), &())
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn project_update(
        &self,
        project_id: &ProjectUuid,
        request: &UpdateProject,
    ) -> Result<()> {
        self.put(format!("projects/{project_id}"), request)
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn project_update_job(
        &self,
        project_id: &ProjectUuid,
        job_id: &ProjectJobUuid,
        request: &UpdateMarketBulk,
    ) -> Result<()> {
        self.put(format!("projects/{project_id}/jobs/{job_id}"), request)
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn project_update_market_bulk(
        &self,
        project_id: &ProjectUuid,
        request: &UpdateMarketBulk,
    ) -> Result<()> {
        self.put(format!("projects/{project_id}/market"), request)
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn project_update_market_entry(
        &self,
        project_id: &ProjectUuid,
        market_id: &MarketUuid,
        request: &UpdateMarketEntry,
    ) -> Result<()> {
        self.put(format!("projects/{project_id}/market/{market_id}"), request)
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn project_update_misc(
        &self,
        project_id: &ProjectUuid,
        request: &UpdateMisc,
    ) -> Result<()> {
        self.put(format!("projects/{project_id}/misc"), request)
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn project_check_materials(
        &self,
        request: &CheckMaterialsRequest,
    ) -> Result<CheckMaterialsResponse> {
        self.post("projects/check", request)
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn project_split_job_check(
        &self,
        project_id: &ProjectUuid,
        request: &SplitJobRequest,
    ) -> Result<SplitJobResponse> {
        self.put(format!("projects/{project_id}/split-job/check"), request)
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn project_update_orderer(
        &self,
        project_id: &ProjectUuid,
        request: String,
    ) -> Result<()> {
        self.put(format!("projects/{project_id}/orderer"), request)
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn project_update_price(
        &self,
        project_id: &ProjectUuid,
        request: f64,
    ) -> Result<()> {
        self.put(format!("projects/{project_id}/price"), request)
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn project_update_sub_project(
        &self,
        project_id: &ProjectUuid,
        request: Vec<ProjectUuid>,
    ) -> Result<()> {
        self.put(format!("projects/{project_id}/sub-project"), request)
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn project_update_notes(
        &self,
        project_id: &ProjectUuid,
        request: String,
    ) -> Result<()> {
        self.put(format!("projects/{project_id}/notes"), request)
            .await
            .map_err(Into::into)
    }
}

/// Different states of the project
///
/// A newly created project will always be in the status `Preparing`.
/// When the projects switches into `InProgress` the job detection gets active
/// for that project.
/// Afterwards the project is either `Done` or `Closed`. Job detection then gets
/// deactivated again.
#[derive(
    Clone,
    Debug,
    Copy,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    sqlx::Type,
    Deserialize,
    Serialize,
    ToSchema,
)]
#[sqlx(type_name = "PROJECT_STATUS")]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProjectStatus {
    /// the project has been created, but no builds have been added
    Draft,
    /// the project has builds selected, and is ready to be started
    ReadyToStart,
    /// the project is currently in progress, and job detection is active
    InProgress,
    /// the project is currently paused, job detection not active
    Paused,
    /// the project is finished, industry job detection is no longer active
    Done,
}

impl ProjectStatus {
    pub fn as_str(&self) -> &str {
        match self {
            ProjectStatus::Done => "Done",
            ProjectStatus::Draft => "Draft",
            ProjectStatus::InProgress => "InProgress",
            ProjectStatus::Paused => "Paused",
            ProjectStatus::ReadyToStart => "ReadyToStart",
        }
    }
}

#[derive(
    Clone,
    Debug,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    sqlx::Type,
    Deserialize,
    Serialize,
    ToSchema,
)]
#[sqlx(type_name = "PROJECT_JOB_STATUS")]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProjectJobStatus {
    WaitingForMaterials,
    ReadyToStart,
    Building,
    Done,
}

#[cfg(test)]
pub async fn project_test_routes(
    postgres: sqlx::PgPool,
    request: axum::http::Request<axum::body::Body>,
) -> axum::http::Response<axum::body::Body> {
    use std::sync::Arc;

    use tower::ServiceExt;

    use crate::metrics::Metric;

    let state = AppState {
        postgres: postgres.clone(),
        metric:   Arc::new(Metric::new()),
    };
    let (app, _) = crate::project::routes(state.clone()).split_for_parts();
    let app = app.with_state(state.clone());

    app.clone()
        .oneshot(request)
        .await
        .unwrap()
}
