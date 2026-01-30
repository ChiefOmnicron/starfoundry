use sqlx::PgPool;
use starfoundry_lib_worker::Task;
use starfoundry_lib_eve_gateway::Blueprint;

use crate::error::{Error, Result};
use crate::metric::WorkerMetric;
use crate::tasks::WorkerEveGatewayTask;

pub async fn insert_blueprints(
    pool:     &PgPool,
    task:     &mut Task<WorkerMetric, WorkerEveGatewayTask>,
    owner_id: i32,
    entries:  Vec<Blueprint>,
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
    let mut material_efficiency = Vec::new();
    let mut time_efficiency     = Vec::new();
    let mut quantity            = Vec::new();
    let mut runs                = Vec::new();

    for entry in entries {
        let location_flag = serde_json::to_string(
                &entry.location_flag
            )
            .unwrap_or("Unknown".into());

        item_ids.push(*entry.item_id as i64);
        location_id.push(*entry.location_id as i64);
        type_ids.push(*entry.type_id);
        location_flags.push(location_flag);
        material_efficiency.push(entry.material_efficiency);
        time_efficiency.push(entry.time_efficiency);
        quantity.push(entry.quantity);
        runs.push(entry.runs);
    }

    let mut transaction = pool
        .begin()
        .await
        .map_err(Error::BeginTransaction)?;

    let update_start = std::time::Instant::now();
    let result = sqlx::query!("
            INSERT INTO asset_blueprint
            (
                owner_id,

                item_id,
                location_id,
                type_id,

                location_flag,
                material_efficiency,
                time_efficiency,
                quantity,
                runs
            )
            SELECT $1, * FROM UNNEST(
                $2::BIGINT[],
                $3::BIGINT[],
                $4::INTEGER[],
                $5::VARCHAR[],
                $6::INTEGER[],
                $7::INTEGER[],
                $8::INTEGER[],
                $9::INTEGER[]
            )
            ON CONFLICT (item_id)
            DO UPDATE SET
                location_id = EXCLUDED.location_id,
                location_flag = EXCLUDED.location_flag,
                material_efficiency = EXCLUDED.material_efficiency,
                time_efficiency = EXCLUDED.time_efficiency,
                quantity = EXCLUDED.quantity,
                runs = EXCLUDED.runs
        ",
            owner_id,

            &item_ids,
            &location_id,
            &type_ids,
            &location_flags,
            &material_efficiency,
            &time_efficiency,
            &quantity,
            &runs,
        )
        .execute(&mut *transaction)
        .await
        .map_err(|e| Error::InsertBlueprintsError(e, owner_id))?;
    // TODO: refactor to `as_millis_f64()` when https://github.com/rust-lang/rust/issues/122451 is stable
    let update_time = update_start.elapsed().as_millis();
    task.metric.blueprint_insert_row_change(
        owner_id,
        result.rows_affected(),
    );
    task.metric.blueprint_insert_duration(
        owner_id,
        update_time,
    );
    task.append_log(format!("Updates: {}", result.rows_affected()));

    let delete_start = std::time::Instant::now();
    let result = sqlx::query!("
            DELETE FROM asset_blueprint
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
    task.metric.blueprint_delete_row_change(
        owner_id,
        result.rows_affected(),
    );
    task.metric.blueprint_delete_duration(
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
