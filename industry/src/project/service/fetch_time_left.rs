use chrono::Utc;
use sqlx::PgPool;
use starfoundry_lib_eve_gateway::EveGatewayApiClient;
use starfoundry_lib_industry::project::{ProjectJobFilter, ProjectJobStatus, ProjectTimeLeft};
use starfoundry_lib_industry::ProjectUuid;
use starfoundry_lib_types::{CharacterId, TypeId};
use std::time::Duration;

use crate::project::error::Result;
use crate::project::service::list_jobs;

pub async fn fetch_time_left(
    pool:                   &PgPool,
    character_id:           CharacterId,
    project_id:             ProjectUuid,
    eve_gateway_api_client: &impl EveGatewayApiClient,
) -> Result<ProjectTimeLeft> {
    let now = Utc::now().naive_utc().and_utc().timestamp();
    let mut projected_end_date = Utc::now().naive_utc();

    let jobs = list_jobs(
            pool,
            character_id,
            eve_gateway_api_client,
            project_id,
            ProjectJobFilter::default(),
        )
        .await?;

    // the first group that had running jobs
    let mut state: String = "DONE".into();

    for group in jobs {
        let jobs = group
            .entries
            .iter()
            .filter(|x| x.status != ProjectJobStatus::Done)
            .collect::<Vec<_>>();

        // 1. if all jobs are started, skip to the next group
        if jobs.is_empty() {
            continue;
        }

        // set the current group
        if state == "DONE" {
            state = group.header.clone();
        }

        // 2. find the job with the most runs, it will also be the one that takes
        // the most time to build
        let mut entry_with_max_runs = if let Some(x) = jobs.first() {
            x
        } else {
            continue;
        };

        jobs
            .iter()
            .for_each(|x| {
                if x.runs > entry_with_max_runs.runs {
                    entry_with_max_runs = x
                }

                // prefer non running jobs over already running jobs
                if
                    entry_with_max_runs.end_date.is_some() &&
                    x.end_date.is_none() &&
                    x.runs >= entry_with_max_runs.runs
                {
                    entry_with_max_runs = x
                }
            });

        if let Some(x) = entry_with_max_runs.end_date {
            // 3. the job is already in progress
            //    build a diff from now until the project finishes
            let end_date = x.and_utc().timestamp();
            let diff = end_date - now;

            let time_to_build = Duration::from_secs(diff as u64);
            projected_end_date += time_to_build;
        } else {
            // 3. fetch the blueprint to get it's build time
            //    only one blueprint is send over, so using first is safe
            let blueprint_time = if let Some(x) = time_overwrite(entry_with_max_runs.item.type_id) {
                x
            } else {
                let blueprint = eve_gateway_api_client
                    .fetch_blueprint_dependencies_bulk(vec![
                        entry_with_max_runs.item.type_id,
                    ])
                    .await?;
                let blueprint = if let Some(x) = blueprint.first() {
                    x
                } else {
                    continue;
                };
                blueprint.time
            };

            // 4. calculate the time it takes, the time is calculated without
            //    bonuses applied
            let time_to_build = blueprint_time * entry_with_max_runs.runs;
            let time_to_build = Duration::from_secs(time_to_build as u64);
            projected_end_date += time_to_build;
        }
    }

    Ok(ProjectTimeLeft {
        date_ms:    projected_end_date.and_utc().timestamp_millis(),
        state:      state,
    })
}

/// Capitals have a very long un-bonused time. To give a more realistic time
/// frame, the build time for capitals is overwritten
fn time_overwrite(type_id: TypeId) -> Option<i32> {
    match *type_id {
        // Revelation
        19720 |
        // Phoenix
        19726 |
        // Moros
        19724 |
        // Naglfar
        19722 => Some(60 * 60 * 24 * 7),  // 7 days in seconds

        // Revelation Navy Issue
        73790 |
        // Phoenix Navy Issue
        73793 |
        // Moros Navy Issue
        73792 |
        // Naglfar Fleet Issue
        73787 |
        // Zirnitra
        52907 |
        // Caiman
        45647 |
        // Chemosh
        42243 |
        // Sarathiel
        87381 |
        // Vehement
        42124 => Some(60 * 60 * 24 * 9),  // 9 days in seconds

        // Bane
        77283 |
        // Karura
        77284 |
        // Hubris
        77281 |
        // Valravn
        77288 => Some(60 * 60 * 24 * 7 * 2), // 2 weeks in seconds

        // Archon
        23757 |
        // Chimera
        23915 |
        // Thanatos
        23911 |
        // Nidhoggur
        24483 => Some(60 * 60 * 24 * 7), // 7 days in seconds

        // Apostle
        37604 |
        // Minokawa
        37605 |
        // Ninazu
        37607 |
        // Lif
        37606 |
        // Dagon
        42242 |
        // Loggerhead
        45645 => Some(60 * 60 * 24 * 7), // 7 days in seconds

        // Aeon
        23919 |
        // Wyvern
        23917 |
        // Nyx
        23913 |
        // Hel
        22852 |
        // Revenant
        3514  |
        // Vendetta
        42125 => Some(60 * 60 * 24 * 7 * 3), // 3 weeks in seconds

        // Avatar
        11567 |
        // Leviathan
        3764  |
        // Erebus
        671   |
        // Ragnarok
        23773 |
        // Azariel
        78576 |
        // Komodo
        45649 |
        // Molok
        42241 |
        // Vanquisher
        42126 => Some(60 * 60 * 24 * 7 * 4), // 4 weeks in seconds

        // Avalanche
        81040 |
        // Charon
        20185 |
        // Fenrir
        20189 |
        // Obelisk
        20187 |
        // Providence
        20183 => Some(60 * 60 * 24 * 7), // 7 days in seconds

        // Anshar
        28848 |
        // Ark
        28850 |
        // Nomad
        28846 |
        // Rhea
        28844 => Some(60 * 60 * 24 * 14), // 14 days in seconds
        _     => None
    }
}
