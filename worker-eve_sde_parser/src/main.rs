//! Parses parts of the EVE provided SDE-File into SQL-Statements for the main
//! application.

use sqlx::postgres::PgPoolOptions;
use starfoundry_lib_eve_sde_parser::Error;
use starfoundry_lib_types::SystemId;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::time::Instant;
use tracing_subscriber::EnvFilter;
use pathfinding::directed::dijkstra::dijkstra;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let pg_addr = std::env::var("STARFOUNDRY_EVE_GATEWAY_DATABASE_URL")
        .expect("Expected that a STARFOUNDRY_EVE_GATEWAY_DATABASE_URL ENV is set");
    let pool = PgPoolOptions::new()
        .min_connections(20)
        .connect(&pg_addr)
        .await
        .unwrap();

    let start = Instant::now();

    starfoundry_lib_eve_sde_parser::import_sde(&pool, None).await?;

    let mut all_systems: HashMap<SystemId, Vec<SystemDistance>> = HashMap::new();
    sqlx::query!("
            SELECT system_start, system_end, distance_ly, s.x, s.y, s.z
            FROM system_distance sd
            JOIN system s ON s.system_id = sd.system_start
        ")
        .fetch_all(&pool)
        .await
        .unwrap()
        .into_iter()
        .map(|x| SystemDistance {
            distance_ly: (x.distance_ly * 10_000f64).ceil() as u32,
            system_id_start: SystemId(x.system_start),
            system_id_end: SystemId(x.system_end),
        })
        .for_each(|x| {
            all_systems
                .entry(x.system_id_start)
                .and_modify(|y: &mut Vec<SystemDistance>| y.push(x.clone()))
                .or_insert(vec![x]);
        });

    // UALX
    let start = 30004807;
    // Mai
    let end = 30003499;
    // ABE
    // Should be: 3.528 -> is: 3.3377 -> 0.1903
    let end = 30004831;
    // Windy
    // Should be: 3.611 -> is: 3.1456 -> 0.4654
    //let end = 30004822;
    // IL-
    // Should be: 3.123 -> is: 2.9542 -> 0.1688
    //let end = 30004832;

    //tracing::info!("Total run time: {}ms", start.elapsed().as_millis());

    let mut graph: HashMap<i32, Vec<(i32, u32)>> = HashMap::new();
    for (system, connections) in all_systems {
        for connection in connections {
            graph
                .entry(*system)
                .and_modify(|x: &mut Vec<(i32, u32)>| x.push((*connection.system_id_end, connection.distance_ly)))
                .or_insert(vec![(*connection.system_id_end, connection.distance_ly)]);
        }
    }

    let successors = |node: &i32| -> Vec<(i32, u32)> {
        graph.get(node).cloned().unwrap_or_default()
    };
    let result = dijkstra(&start, successors, |&node| node == end);

    match result {
        Some((path, cost)) => {
            dbg!(&path, cost as f64 / 10_000f64);
            for system in path {
                let system_db = sqlx::query!("
                        SELECT system_name
                        FROM system
                        WHERE system_id = $1
                    ",
                        system,
                    )
                    .fetch_one(&pool)
                    .await
                    .unwrap();

                let system_name = system_db.system_name;
                dbg!(system_name);
            }
        }
        None => println!("No path found"),
    }

    Ok(())
}

#[derive(Clone, Debug)]
struct SystemDistance {
    system_id_start:    SystemId,
    system_id_end:      SystemId,
    distance_ly:        u32,
}
