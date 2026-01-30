use sqlx::PgPool;
use starfoundry_lib_worker::Task;
use starfoundry_lib_eve_gateway::Asset;

use crate::error::{Error, Result};
use crate::metric::WorkerMetric;
use crate::tasks::WorkerEveGatewayTask;

pub async fn insert_assets(
    pool:     &PgPool,
    task:     &mut Task<WorkerMetric, WorkerEveGatewayTask>,
    owner_id: i32,
    entries:  Vec<Asset>,
) -> Result<()> {
    if entries.is_empty() {
        return Ok(());
    }

    let mut entries = entries;
    entries.sort_by(|a, b| a.item_id.cmp(&b.item_id));
    entries.dedup_by_key(|x| x.item_id);

    let mut item_ids            = Vec::new();
    let mut location_id         = Vec::new();
    let mut type_ids            = Vec::new();
    let mut location_flags      = Vec::new();
    let mut location_type       = Vec::new();
    let mut quantity            = Vec::new();
    let mut is_singleton        = Vec::new();
    let mut is_blueprint_copy   = Vec::new();

    for entry in entries {
        let location_flag = serde_json::to_string(
                &entry.location_flag
            )
            .unwrap_or("Unknown".into());

        item_ids.push(*entry.item_id as i64);
        location_id.push(*entry.location_id as i64);
        type_ids.push(*entry.type_id);
        location_flags.push(location_flag);
        location_type.push(entry.location_type);
        quantity.push(entry.quantity);
        is_singleton.push(entry.is_singleton);
        is_blueprint_copy.push(entry.is_blueprint_copy);
    }

    let mut transaction = pool
        .begin()
        .await
        .map_err(Error::BeginTransaction)?;

    let update_start = std::time::Instant::now();
    let result = sqlx::query!("
            INSERT INTO asset
            (
                owner_id,

                item_id,
                location_id,
                type_id,

                location_flag,
                location_type,
                quantity,
                is_singleton,
                is_blueprint_copy
            )
            SELECT $1, * FROM UNNEST(
                $2::BIGINT[],
                $3::BIGINT[],
                $4::INTEGER[],
                $5::VARCHAR[],
                $6::VARCHAR[],
                $7::INTEGER[],
                $8::BOOLEAN[],
                $9::BOOLEAN[]
            )
            ON CONFLICT (item_id)
            DO UPDATE SET
                location_id = EXCLUDED.location_id,
                location_flag = EXCLUDED.location_flag,
                quantity = EXCLUDED.quantity,
                is_singleton = EXCLUDED.is_singleton
        ",
            owner_id,

            &item_ids,
            &location_id,
            &type_ids,
            &location_flags,
            &location_type,
            &quantity,
            &is_singleton,
            &is_blueprint_copy as _,
        )
        .execute(&mut *transaction)
        .await
        .map_err(|e| Error::InsertAssetError(e, owner_id))?;
    // TODO: refactor to `as_millis_f64()` when https://github.com/rust-lang/rust/issues/122451 is stable
    let update_time = update_start.elapsed().as_millis();
    task.metric.asset_insert_row_change(
        owner_id,
        result.rows_affected(),
    );
    task.metric.asset_insert_duration(
        owner_id,
        update_time,
    );
    task.append_log(format!("Updates: {}", result.rows_affected()));

    let delete_start = std::time::Instant::now();
    let result = sqlx::query!("
            DELETE FROM asset
            WHERE owner_id = $1
            AND NOT item_id = ANY($2)
        ",
            owner_id,
            &item_ids,
        )
        .execute(&mut *transaction)
        .await
        .map_err(|e| Error::CleanupItems(e, owner_id))?;
    // TODO: refactor to `as_millis_f64()` when https://github.com/rust-lang/rust/issues/122451 is stable
    let delete_time = delete_start.elapsed().as_millis();
    task.metric.asset_delete_row_change(
        owner_id,
        result.rows_affected(),
    );
    task.metric.asset_delete_duration(
        owner_id,
        delete_time,
    );
    task.append_log(format!("Deletes: {}", result.rows_affected()));

    transaction
        .commit()
        .await
        .map_err(Error::CommitTransaction)?;

    Ok(())
}
