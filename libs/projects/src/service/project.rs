use chrono::NaiveDateTime;
use sqlx::PgPool;
use starfoundry_libs_structures::StructureUuid;
use starfoundry_libs_types::CharacterId;

use crate::{ActiveJob, AddMarket, AddMisc, CheckResources, CostEstimateConfiguration, CostEstimateResponse, CreateProject, Error, Excess, FetchJobFilter, Finance, Job, Market, MarketRecommendation, Misc, Product, Project, ProjectFilter, ProjectJobUuid, ProjectMarketUuid, ProjectMiscUuid, ProjectUuid, Result, StartableJob, Stock, StockMinimal, UpdateExcessPrice, UpdateJob, UpdateMarket, UpdateMineral, UpdateMisc, UpdateProject, UpdateStockPrice};

pub struct ProjectService(ProjectUuid);

impl ProjectService {
    pub fn new(
        project_uuid: ProjectUuid,
    ) -> Self {
        ProjectService(project_uuid)
    }

    pub async fn assert_owner(
        &self,
        pool:         &PgPool,
        character_id: CharacterId,
    ) -> Result<()> {
        let result = sqlx::query!("
                SELECT id
                FROM project
                WHERE id = $1
                AND owner = $2
            ",
                *self.0,
                *character_id,
            )
            .fetch_optional(pool)
            .await
            .map_err(|e| Error::FetchPermissions(e, self.0))?;

        if result.is_none() {
            return Err(Error::Forbidden(*self.0, character_id));
        } else {
            Ok(())
        }
    }

    pub async fn assert_read_access(
        &self,
        pool:         &PgPool,
        character_id: CharacterId,
    ) -> Result<()> {
        let result = sqlx::query!("
                SELECT p.id
                FROM project p
                JOIN project_group_member pgm ON pgm.group_id = p.project_group_id
                WHERE p.id = $1
                AND (
                    pgm.character_id = $2 OR
                    p.owner = $2
                )
                AND (
                    pgm.projects = 'WRITE' OR
                    pgm.projects = 'READ'
                )
            ",
                *self.0,
                *character_id,
            )
            .fetch_optional(pool)
            .await
            .map_err(|e| Error::FetchPermissions(e, self.0))?;

        if result.is_none() {
            return Err(Error::Forbidden(*self.0, character_id));
        } else {
            Ok(())
        }
    }

    pub async fn assert_write_access(
        &self,
        pool:         &PgPool,
        character_id: CharacterId,
    ) -> Result<()> {
        let result = sqlx::query!("
                SELECT p.id
                FROM project p
                JOIN project_group_member pgm ON pgm.group_id = p.project_group_id
                WHERE p.id = $1
                AND (
                    pgm.character_id = $2 OR
                    p.owner = $2
                )
                AND pgm.projects = 'WRITE'
            ",
                *self.0,
                *character_id,
            )
            .fetch_optional(pool)
            .await
            .map_err(|e| Error::FetchPermissions(e, self.0))?;

        if result.is_none() {
            return Err(Error::Forbidden(*self.0, character_id));
        } else {
            Ok(())
        }
    }

    pub async fn assert_exists(
        &self,
        pool: &PgPool,
    ) -> Result<()> {
        let project = sqlx::query!("
                SELECT id
                FROM project
                WHERE id = $1
            ",
                *self.0,
            )
            .fetch_optional(pool)
            .await
            .map_err(|e| Error::FetchProject(e, self.0))?;

        if project.is_some() {
            Ok(())
        } else {
            Err(Error::ProjectNotFound(self.0))
        }
    }

    pub async fn create(
        pool:         &PgPool,
        character_id: CharacterId,
        project_data: CreateProject,
    ) -> Result<ProjectUuid> {
        crate::root::create(
                pool,
                character_id,
                project_data,
            )
            .await
    }

    pub async fn delete(
        &self,
        pool:         &PgPool,
        character_id: CharacterId,
    ) -> Result<()> {
        self.assert_exists(pool).await?;
        self.assert_owner(pool, character_id).await?;

        crate::root::delete(
                pool,
                self.0,
            )
            .await
    }

    pub async fn list(
        pool:         &PgPool,
        character_id: CharacterId,
        filter:       ProjectFilter,
    ) -> Result<Vec<ProjectUuid>> {
        crate::root::list(
                pool,
                character_id,
                filter,
            )
            .await
    }

    pub async fn update(
        &self,
        pool:         &PgPool,
        character_id: CharacterId,
        update:       UpdateProject,
    ) -> Result<()> {
        self.assert_exists(pool).await?;
        self.assert_write_access(pool, character_id).await?;

        crate::root::update(
                pool,
                self.0,
                update,
            )
            .await
    }

    pub async fn fetch(
        &self,
        pool:         &PgPool,
        character_id: CharacterId,
    ) -> Result<Option<Project>> {
        self.assert_exists(pool).await?;
        self.assert_read_access(pool, character_id).await?;

        crate::root::fetch(
                pool,
                self.0,
            )
            .await
    }

    pub async fn fetch_excess(
        &self,
        pool:         &PgPool,
        character_id: CharacterId,
    ) -> Result<Excess> {
        self.assert_exists(pool).await?;
        self.assert_read_access(pool, character_id).await?;

        crate::excess::fetch(
                pool,
                self.0
            )
            .await
    }

    pub async fn update_excess_price(
        &self,
        pool:         &PgPool,
        character_id: CharacterId,
        update:       UpdateExcessPrice,
    ) -> Result<()> {
        self.assert_exists(pool).await?;
        self.assert_write_access(pool, character_id).await?;

        crate::excess::update_price(
                pool,
                self.0,
                update
            )
            .await
    }

    pub async fn fetch_finance(
        &self,
        pool:         &PgPool,
        character_id: CharacterId,
    ) -> Result<Option<Finance>> {
        self.assert_exists(pool).await?;
        self.assert_read_access(pool, character_id).await?;

        crate::finance::fetch(
                pool,
                self.0
            )
            .await
    }

    pub async fn active_jobs(
        &self,
        pool:         &PgPool,
        character_id: CharacterId,
    ) -> Result<Vec<ActiveJob>> {
        self.assert_exists(pool).await?;
        self.assert_read_access(pool, character_id).await?;

        crate::job::active_jobs(
                pool,
                self.0,
            )
            .await
    }

    pub async fn startable_jobs(
        &self,
        pool:         &PgPool,
        character_id: CharacterId,
    ) -> Result<StartableJob> {
        self.assert_exists(pool).await?;
        self.assert_read_access(pool, character_id).await?;

        crate::job::startable_jobs(
                pool,
                self.0,
            )
            .await
    }

    pub async fn fetch_jobs(
        &self,
        pool:         &PgPool,
        character_id: CharacterId,
        filter:       FetchJobFilter,
    ) -> Result<Job> {
        self.assert_exists(pool).await?;
        self.assert_read_access(pool, character_id).await?;

        crate::job::fetch(
                pool,
                self.0,
                filter,
            )
            .await
    }

    pub async fn delete_job(
        &self,
        pool:         &PgPool,
        character_id: CharacterId,
        job_uuid:     ProjectJobUuid,
    ) -> Result<()> {
        self.assert_exists(pool).await?;
        self.assert_write_access(pool, character_id).await?;

        crate::job::delete(
                pool,
                self.0,
                job_uuid,
            )
            .await
    }

    pub async fn update_job(
        &self,
        pool:         &PgPool,
        character_id: CharacterId,
        job_uuid:     ProjectJobUuid,
        update:       UpdateJob,
    ) -> Result<()> {
        self.assert_exists(pool).await?;
        self.assert_write_access(pool, character_id).await?;

        crate::job::update(
                pool,
                self.0,
                job_uuid,
                update,
            )
            .await
    }

    pub async fn fetch_market(
        &self,
        pool:         &PgPool,
        character_id: CharacterId,
    ) -> Result<Market> {
        self.assert_exists(pool).await?;
        self.assert_read_access(pool, character_id).await?;

        crate::market::fetch(
                pool,
                self.0,
            )
            .await
    }

    pub async fn fetch_market_prices(
        &self,
        pool:         &PgPool,
        character_id: CharacterId,
    ) -> Result<Vec<MarketRecommendation>> {
        self.assert_exists(pool).await?;
        self.assert_read_access(pool, character_id).await?;

        crate::market::fetch_prices(
                pool,
                self.0,
            )
            .await
    }

    pub async fn fetch_market_prices_gas(
        &self,
        pool:         &PgPool,
        character_id: CharacterId,
    ) -> Result<Vec<MarketRecommendation>> {
        self.assert_exists(pool).await?;
        self.assert_read_access(pool, character_id).await?;

        crate::market::fetch_gas(
                pool,
                self.0,
            )
            .await
    }

    pub async fn fetch_market_prices_minerals(
        &self,
        pool:         &PgPool,
        character_id: CharacterId,
    ) -> Result<Vec<MarketRecommendation>> {
        self.assert_exists(pool).await?;
        self.assert_read_access(pool, character_id).await?;

        crate::market::fetch_minerals(
                pool,
                self.0,
            )
            .await
    }

    pub async fn last_market_fetch(
        pool:           &PgPool,
        sturcture_uuid: StructureUuid,
    ) -> Result<Option<NaiveDateTime>> {
        crate::market::last_fetch(
                pool,
                sturcture_uuid,
            )
            .await
    }

    pub async fn add_market(
        &self,
        pool:         &PgPool,
        character_id: CharacterId,
        entry:        AddMarket,
    ) -> Result<()> {
        self.assert_exists(pool).await?;
        self.assert_write_access(pool, character_id).await?;

        crate::market::add(
                pool,
                &self.0,
                entry,
            )
            .await
    }

    pub async fn delete_market(
        &self,
        pool:         &PgPool,
        character_id: CharacterId,
        market_uuid:  ProjectMarketUuid,
    ) -> Result<()> {
        self.assert_exists(pool).await?;
        self.assert_write_access(pool, character_id).await?;

        crate::market::delete(
                pool,
                self.0,
                market_uuid,
            )
            .await
    }

    pub async fn update_market(
        &self,
        pool:         &PgPool,
        character_id: CharacterId,
        market_uuid:  ProjectMarketUuid,
        updates:      UpdateMarket,
    ) -> Result<()> {
        self.assert_exists(pool).await?;
        self.assert_write_access(pool, character_id).await?;

        crate::market::update(
                pool,
                self.0,
                market_uuid,
                updates,
            )
            .await
    }

    pub async fn update_bulk_market(
        &self,
        pool:         &PgPool,
        character_id: CharacterId,
        updates:      Vec<UpdateMarket>,
    ) -> Result<()> {
        self.assert_exists(pool).await?;
        self.assert_write_access(pool, character_id).await?;

        crate::market::update_bulk(
                pool,
                self.0,
                updates,
            )
            .await
    }

    pub async fn add_misc(
        &self,
        pool:         &PgPool,
        character_id: CharacterId,
        entry:        AddMisc,
    ) -> Result<()> {
        self.assert_exists(pool).await?;
        self.assert_write_access(pool, character_id).await?;

        crate::misc::add(
                pool,
                self.0,
                entry,
            )
            .await
    }

    pub async fn delete_misc(
        &self,
        pool:         &PgPool,
        character_id: CharacterId,
        misc_uuid:    ProjectMiscUuid,
    ) -> Result<()> {
        self.assert_exists(pool).await?;
        self.assert_write_access(pool, character_id).await?;

        crate::misc::delete(
                pool,
                self.0,
                misc_uuid,
            )
            .await
    }

    pub async fn fetch_misc(
        &self,
        pool:         &PgPool,
        character_id: CharacterId,
    ) -> Result<Vec<Misc>> {
        self.assert_exists(pool).await?;
        self.assert_read_access(pool, character_id).await?;

        crate::misc::fetch(
                pool,
                self.0,
            )
            .await
    }

    pub async fn update_misc(
        &self,
        pool:         &PgPool,
        character_id: CharacterId,
        update:       UpdateMisc,
    ) -> Result<()> {
        self.assert_exists(pool).await?;
        self.assert_write_access(pool, character_id).await?;

        crate::misc::update(
                pool,
                self.0,
                update,
            )
            .await
    }

    pub async fn fetch_product(
        &self,
        pool:         &PgPool,
        character_id: CharacterId,
    ) -> Result<Vec<Product>> {
        self.assert_exists(pool).await?;
        self.assert_read_access(pool, character_id).await?;

        crate::product::fetch(
                pool,
                self.0,
            )
            .await
    }

    pub async fn update_market_minerals(
        &self,
        pool:         &PgPool,
        character_id: CharacterId,
        updates:      Vec<UpdateMineral>,
    ) -> Result<()> {
        self.assert_exists(pool).await?;
        self.assert_write_access(pool, character_id).await?;

        crate::market::update_minerals(
                pool,
                self.0,
                updates,
            )
            .await
    }

    pub async fn fetch_stock(
        &self,
        pool:         &PgPool,
        character_id: CharacterId,
    ) -> Result<Stock> {
        self.assert_exists(pool).await?;
        self.assert_read_access(pool, character_id).await?;

        crate::stock::fetch(
                pool,
                self.0
            )
            .await
    }

    pub async fn update_stock_price(
        &self,
        pool:         &PgPool,
        character_id: CharacterId,
        update:       UpdateStockPrice,
    ) -> Result<()> {
        self.assert_exists(pool).await?;
        self.assert_write_access(pool, character_id).await?;

        crate::stock::update_price(
                pool,
                self.0,
                update,
            )
            .await
    }

    pub async fn check_resources(
        pool:           &PgPool,
        resources_jobs: CheckResources,
    ) -> Result<Vec<StockMinimal>> {
        crate::root::check_resources(
                pool,
                resources_jobs,
            ).await
    }

    pub async fn cost_estimate(
        pool:         &PgPool,
        character_id: CharacterId,
        config:       CostEstimateConfiguration,
    ) -> Result<CostEstimateResponse> {
        crate::root::cost_estimate(
                pool,
                character_id,
                config,
            ).await
    }
}
