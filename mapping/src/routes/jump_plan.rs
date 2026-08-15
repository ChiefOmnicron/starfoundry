use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use pathfinding::directed::dijkstra::dijkstra;
use sqlx::PgPool;
use starfoundry_lib_mapping::{CreateJumpPlan, JumpPlanEntry};
use std::collections::HashMap;
use starfoundry_lib_types::SystemId;

use crate::api_docs::{InternalServerError, NotFound};
use crate::routes::error::Result;
use crate::state::AppState;

/// Fetch Route
/// 
/// - Alternative route: `/latest/routes/jump-plans`
/// - Alternative route: `/v1/routes/jump-plans`
/// 
/// ---
/// 
/// Plans a route based on the given information
/// 
#[utoipa::path(
    get,
    path = "/jump-plans",
    tag = "Route",
    responses(
        (
            body = JumpPlanEntry,
            description = "Information about the jump plan",
            status = OK,
        ),
        NotFound,
        InternalServerError,
    ),
)]
pub async fn api(
    State(state):   State<AppState>,
    Json(body):     Json<CreateJumpPlan>,
) -> Result<impl IntoResponse> {
    Ok((
        StatusCode::OK,
    ))
}

pub async fn calculate_jump_plan(
    pool:       &PgPool,
    jump_plan:  CreateJumpPlan,
) -> Result<Vec<JumpPlanEntry>> {
    type SystemIdDijkstra = i32;

    let mut all_systems: HashMap<SystemId, Vec<SystemDistanceTmp>> = HashMap::new();
    let systems = sqlx::query!("
            SELECT
                system_start,
                system_end,
                distance_ly
            FROM system_distance_cache
            WHERE distance_ly <= $1
            AND NOT (system_start = ANY($2))
        ",
            jump_plan.max_distance_ly,
            &jump_plan.blacklist_system_ids.iter().map(|x| **x).collect::<Vec<_>>(),
        )
        .fetch_all(pool)
        .await
        .unwrap();

    // TODO: check if a structure limit is set
    let systems = if true {
        let filtered_systems = sqlx::query!("
                SELECT system_id
                FROM structure
                WHERE system_id = ANY($1)
                AND type_id = ANY(ARRAY[35834, 35833, 35832])
            ",
                &systems.iter().map(|x| x.system_start).collect::<Vec<_>>(),
            )
            .fetch_all(pool)
            .await
            .unwrap()
            .into_iter()
            .map(|x| x.system_id)
            .collect::<Vec<_>>();
        systems
            .into_iter()
            .filter(|x| filtered_systems.contains(&x.system_start) && filtered_systems.contains(&x.system_end))
            .collect::<Vec<_>>()
    } else {
        systems
    };

    systems
        .into_iter()
        .map(|x| SystemDistanceTmp {
            distance_ly: (x.distance_ly * 10_000f32).ceil() as u32,
            system_start: SystemId(x.system_start),
            system_end: SystemId(x.system_end),
        })
        .for_each(|x| {
            all_systems
                .entry(x.system_start)
                .and_modify(|y: &mut Vec<SystemDistanceTmp>| y.push(x.clone()))
                .or_insert(vec![x]);
        });

    let mut graph: HashMap<SystemIdDijkstra, Vec<(SystemIdDijkstra, u32)>> = HashMap::new();
    for (system, connections) in all_systems {
        for connection in connections {
            graph
                .entry(*system)
                .and_modify(|x: &mut Vec<(SystemIdDijkstra, u32)>| x.push((*connection.system_end, connection.distance_ly)))
                .or_insert(vec![(*connection.system_end, connection.distance_ly)]);
        }
    }

    let successors = |node: &SystemIdDijkstra| -> Vec<(SystemIdDijkstra, u32)> {
        graph.get(node).cloned().unwrap_or_default()
    };

    let mut start_system_id = jump_plan.system_start_id;
    let mut intermediate_system_ids = Vec::new();
    intermediate_system_ids.extend(jump_plan.intermediate_system_ids);
    intermediate_system_ids.push(jump_plan.system_end_id);

    let mut jumps = Vec::new();

    for intermediate in intermediate_system_ids {
        let end_system_id = intermediate;

        if start_system_id == end_system_id {
            continue;
        }

        let result = dijkstra(
            &*start_system_id,
            successors,
            |&node| node == *end_system_id
        );

        match result {
            Some((path, cost)) => {
                dbg!(&cost);
                let mut intermediate_start_system_id = start_system_id;

                for system in path {
                    if start_system_id == system.into() {
                        intermediate_start_system_id = system.into();
                        continue;
                    }

                    let jump = JumpPlanEntry {
                        system_id_start:    intermediate_start_system_id,
                        system_id_end:      system.into(),
                        distance:           (cost / 10_000) as i32,
                    };
                    jumps.push(jump);

                    intermediate_start_system_id = system.into();
                }
            }
            None => (),
        }

        start_system_id = end_system_id;
    }

    Ok(jumps)
}

#[derive(Clone, Debug)]
struct SystemDistanceTmp {
    distance_ly:    u32,
    system_start:   SystemId,
    system_end:     SystemId,
}

mod test {
    use sqlx::postgres::PgPoolOptions;
    use starfoundry_lib_mapping::CreateJumpPlan;

    use super::calculate_jump_plan;

    #[tokio::test]
    async fn route_test() {
        let pool = PgPoolOptions::new()
            .connect("postgresql://postgres:postgres@localhost:5432/dev-sf-mapping")
            .await
            .unwrap();

        let jump_plan_entries = calculate_jump_plan(
                &pool,
                CreateJumpPlan {
                    intermediate_system_ids: vec![
                        //30000018.into(),
                    ],
                    blacklist_system_ids: vec![
                        30001231.into(),
                    ],
                    max_distance_ly: 10f32,
                    //system_end_id: 30000772.into(), // C-J
                    system_end_id: 30003499.into(), // C-J
                    //system_start_id: 30004807.into(), // UALX
                    system_start_id: 30004831.into(), // UALX
                },
            )
            .await
            .unwrap();

        for entry in jump_plan_entries {
            let start_system = sqlx::query!("
                    SELECT system_name
                    FROM system_cache
                    WHERE system_id = $1
                ",
                    *entry.system_id_start,
                )
                .fetch_one(&pool)
                .await
                .unwrap()
                .system_name;
            let end_system = sqlx::query!("
                    SELECT system_name
                    FROM system_cache
                    WHERE system_id = $1
                ",
                    *entry.system_id_end,
                )
                .fetch_one(&pool)
                .await
                .unwrap()
                .system_name;
            let distance_ly = sqlx::query!("
                    SELECT distance_ly
                    FROM system_distance_cache
                    WHERE system_start = $1
                    AND system_end = $2
                ",
                    *entry.system_id_start,
                    *entry.system_id_end,
                )
                .fetch_one(&pool)
                .await
                .unwrap()
                .distance_ly;

            println!("{} -> {} ({})", start_system, end_system, distance_ly);
        }
        panic!("asdasd");
    }
}
