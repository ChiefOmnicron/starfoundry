use sqlx::PgPool;

use crate::error::{Error, Result};
use crate::task::Task;

pub async fn task(
    _tasl: &mut Task,
    pool:  &PgPool,
) -> Result<()> {
        sqlx::query!("
                DELETE FROM appraisal
                WHERE DATE(created_at) < DATE(NOW() - INTERVAL '90 DAY')
            ")
            .execute(pool)
            .await
            .map_err(Error::DeleteAppraisals)?;
    Ok(())
}
