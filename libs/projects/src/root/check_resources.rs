use sqlx::PgPool;
use starfoundry_libs_structures::{StructureService, StructureUuid};
use starfoundry_libs_types::TypeId;
use std::collections::HashMap;

use crate::{CheckResources, Error, ProjectStructureGroup, Result, StockMinimal, StructureMapping};
use crate::engine::{CalculationEngine, Dependency, ProjectConfigBuilder};

pub async fn check_resources(
    pool:           &PgPool,
    resources_jobs: CheckResources,
) -> Result<Vec<StockMinimal>> {
    let mut required_resources   = HashMap::new();
    let mut grouped_by_structure = HashMap::new();

    sqlx::query_as!(
        JobToStart,
        "
            SELECT
                type_id,
                runs,
                structure_id
            FROM project_job
            WHERE id = ANY($1)
        ",
            &resources_jobs.job_ids.iter().map(|x| **x).collect::<Vec<_>>(),
        )
        .fetch_all(pool)
        .await
        .map_err(|e| Error::FetchJobsByArray(e, resources_jobs.job_ids.clone()))?
        .into_iter()
        .for_each(|job| {
            grouped_by_structure
                .entry(job.structure_id)
                .and_modify(|x: &mut Vec<JobToStart>| x.push(job.clone()))
                .or_insert(vec![job]);
        });

    for (structure_uuid, jobs) in grouped_by_structure {
        let mut group = ProjectStructureGroup::default();

        let structure_service = StructureService::new(structure_uuid);
        let structure = structure_service
            .danger_no_permission_fetch(pool)
            .await?
            .ok_or_else(|| Error::StructureNotFound(structure_uuid))?;

        group
            .mapping
            .push(StructureMapping {
                structure_uuid: structure.id,
                category_group: structure.category_groups(),
            });

        let config = ProjectConfigBuilder::default()
            .add_structures(vec![
                structure,
            ])
            .add_structure_mappings(group.mapping)
            .set_skip_chidren(true)
            .build();

        let mut engine = CalculationEngine::new(config);
        for job in jobs.iter() {
            let type_id = job.type_id;
            let json = sqlx::query!("
                    SELECT data
                    FROM   blueprint_json
                    WHERE  ptype_id = $1
                ",
                    *type_id
                )
                .fetch_one(pool)
                .await
                .unwrap()
                .data;
            let json = serde_json::to_value(&json).unwrap();

            if let Ok(x) = Dependency::try_from(job.runs as u32, json) {
                engine.add(x);
            } else {
                continue;
            };
        }

        let mut tree = engine;
        let dependency_result = tree
            .apply_bonus()
            .finalize();

        for (_, job) in dependency_result.tree {
            for (type_id, materials) in job.children {
                for run in job.runs.iter() {
                    required_resources
                        .entry(type_id)
                        .and_modify(|x: &mut i32| *x += (materials * *run as f32).ceil() as i32)
                        .or_insert((materials * *run as f32).ceil() as i32);
                }
            }
        }
    }

    for resource in resources_jobs.resources {
        required_resources
            .entry(resource.type_id)
            .and_modify(|x: &mut i32| *x -= resource.quantity);
    }

    let required_resources = required_resources
        .into_iter()
        .map(|(type_id, quantity)| StockMinimal {
            quantity,
            type_id,
        })
        .collect::<Vec<_>>();

    Ok(required_resources)
}

#[derive(Clone, Debug)]
struct JobToStart {
    type_id:      TypeId,
    runs:         i32,
    structure_id: StructureUuid,
}
