use sqlx::PgPool;

pub async fn migrate_industry_jobs(
    postgres_source:      &PgPool,
    postgres_destination: &PgPool,
) -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!("Start - industry jobs");
    let industry_jobs = sqlx::query!(r#"
            SELECT
                blueprint_id,
                blueprint_location_id,
                blueprint_type_id,
                facility_id,
                installer_id,
                job_id,
                runs,
                cost,
                end_date,
                activity AS "activity!: IndustryActivity",
                is_delivered,
                character_corporation_id,
                ignore
            FROM industry_job
        "#)
        .fetch_all(postgres_source)
        .await?;

    let mut transaction = postgres_destination.begin().await?;
    for industry_job in industry_jobs {
        sqlx::query!("
                INSERT INTO industry_job
                (
                    blueprint_id,
                    blueprint_location_id,
                    blueprint_type_id,
                    facility_id,
                    installer_id,
                    job_id,
                    runs,
                    cost,
                    end_date,
                    activity,
                    is_delivered,
                    character_corporation_id,
                    ignore
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
                ON CONFLICT (job_id)
                DO UPDATE SET
                    is_delivered = EXCLUDED.is_delivered,
                    ignore       = EXCLUDED.ignore
            ",
                industry_job.blueprint_id,
                industry_job.blueprint_location_id,
                industry_job.blueprint_type_id,
                industry_job.facility_id,
                industry_job.installer_id,
                industry_job.job_id,
                industry_job.runs,
                industry_job.cost,
                industry_job.end_date,
                industry_job.activity as _,
                industry_job.is_delivered,
                industry_job.character_corporation_id,
                industry_job.ignore,
            )
            .execute(&mut *transaction)
            .await?;
    }
    transaction.commit().await?;
    tracing::info!("Done - industry jobs");

    Ok(())
}

#[derive(Clone, Debug, sqlx::Type)]
#[sqlx(type_name = "INDUSTRY_ACTIVITY")]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum IndustryActivity {
    Manufacturing,
    TimeEfficiencyResearch,
    MaterialEfficiencyResearch,
    Copying,
    Invention,
    Reactions,
    Unknown,
}
