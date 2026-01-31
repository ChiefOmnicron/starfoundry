use sqlx::PgPool;
use starfoundry_lib_worker::Task;
use uuid::Uuid;

use crate::error::{Error, Result};
use crate::metric::WorkerMetric;
use crate::tasks::WorkerEveGatewayTask;
use std::collections::HashMap;

pub async fn system_index_compress(
    pool: &PgPool,
    task: &mut Task<WorkerMetric, WorkerEveGatewayTask>,
) -> Result<()> {
    #[derive(Clone, Debug,)]
    struct TmpStruct {
        id:                Uuid,
        manufacturing:     f32,
        copying:           f32,
        invention:         f32,
        reaction:          f32,
        reasearch_time:    f32,
        research_material: f32,
        system_id:         i32,
    }

    let system_ids = sqlx::query!("
            SELECT DISTINCT(system_id) FROM system_index
        ")
        .fetch_all(pool)
        .await
        .map_err(|e| {
            task.append_error(format!("{e}"));
            Error::CompressSystemIndex(e)
        })?
        .into_iter()
        .map(|x| x.system_id);

    let mut ids           = Vec::new();
    let mut timestamps    = Vec::new();
    let mut systems       = Vec::new();
    let mut manufacturing = Vec::new();
    let mut reaction      = Vec::new();
    let mut copying       = Vec::new();
    let mut invention     = Vec::new();
    let mut rme           = Vec::new();
    let mut rte           = Vec::new();

    for system_id in system_ids {
        let mut grouped_by_day = HashMap::new();

        sqlx::query!(r#"
                SELECT
                    DATE(timestamp) AS "date!",
                    *
                FROM system_index
                WHERE system_id = $1
                --AND DATE(timestamp) > DATE(NOW() - INTERVAL '7 DAY')
                AND DATE(timestamp) < DATE(NOW() - INTERVAL '3 DAY')
            "#,
                system_id,
            )
            .fetch_all(pool)
            .await
            .map_err(|e| {
                task.append_error(format!("{e}"));
                Error::CompressSystemIndex(e)
            })?
            .into_iter()
            .for_each(|x| {
                let tmp = TmpStruct {
                    id:                x.id,
                    copying:           x.copying,
                    invention:         x.invention,
                    manufacturing:     x.manufacturing,
                    reaction:          x.reaction,
                    reasearch_time:    x.research_time,
                    research_material: x.research_material,
                    system_id:         x.system_id,
                };

                grouped_by_day
                    .entry(x.date)
                    .and_modify(|x: &mut Vec<TmpStruct>| x.push(tmp.clone()))
                    .or_insert(vec![tmp]);
            });

        for (timestamp, entries) in grouped_by_day {
            if entries.len() == 1 {
                continue;
            }

            timestamps.push(timestamp);
            systems.push(entries[0].system_id);

            ids.extend(entries.iter().map(|x| x.id).collect::<Vec<_>>());
            manufacturing.push(entries.iter().map(|x| x.manufacturing).sum::<f32>() / entries.len() as f32);
            reaction.push(entries.iter().map(|x| x.reaction).sum::<f32>() / entries.len() as f32);
            copying.push(entries.iter().map(|x| x.copying).sum::<f32>() / entries.len() as f32);
            invention.push(entries.iter().map(|x| x.invention).sum::<f32>() / entries.len() as f32);
            rme.push(entries.iter().map(|x| x.research_material).sum::<f32>() / entries.len() as f32);
            rte.push(entries.iter().map(|x| x.reasearch_time).sum::<f32>() / entries.len() as f32);
        }
    }

    let mut transaction = pool
        .begin()
        .await
        .map_err(|e| {
            task.append_error(format!("{e}"));
            Error::BeginTransaction(e)
        })?;

    sqlx::query!("
            DELETE FROM system_index
            WHERE id = ANY($1)
        ",
            &ids,
        )
        .execute(&mut *transaction)
        .await
        .map_err(Error::CompressSystemIndex)?;

    sqlx::query!("
            INSERT INTO system_index
            (
                timestamp,
                system_id,
                manufacturing,
                reaction,
                copying,
                invention,
                research_time,
                research_material
            )
            SELECT * FROM UNNEST
            (
                $1::TIMESTAMP[],
                $2::INTEGER[],
                $3::REAL[],
                $4::REAL[],
                $5::REAL[],
                $6::REAL[],
                $7::REAL[],
                $8::REAL[]
            )
            ON CONFLICT (timestamp, system_id) DO UPDATE SET
                manufacturing = EXCLUDED.manufacturing,
                reaction = EXCLUDED.reaction,
                copying = EXCLUDED.copying,
                invention = EXCLUDED.invention,
                research_time = EXCLUDED.research_time,
                research_material = EXCLUDED.research_material
        ",
            &timestamps as _,
            &systems,
            &manufacturing,
            &reaction,
            &copying,
            &invention,
            &rte,
            &rme,
        )
        .execute(&mut *transaction)
        .await
        .map_err(|e| {
            task.append_error(format!("{e}"));
            Error::CompressSystemIndex(e)
        })?;

    transaction
        .commit()
        .await
        .map_err(|e| {
            task.append_error(format!("{e}"));
            Error::CommitTransaction(e)
        })?;

    Ok(())
}
