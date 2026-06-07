use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use starfoundry_lib_types::{ItemId, JobId};
use starfoundry_lib_eve_gateway::IndustryJob;

use crate::jobs::{ProjectJobStatusDatabase, StartableIndustryJobs, UpdateJobRequest};
use tracing::{Level, event, span};

/// Attempts to match an industry job retrieved from the eve api, to a industry
/// job that needs to be done in order to complete a project.
/// 
/// The detection is very strict.
/// A job always needs to go into a container, named after the project, e.g.
/// `C1000` for the project named `C1000` in the application.
/// If this is not the case, the job detection will not attempt to assign them.
/// 
/// # Params
/// 
/// - `eve_jobs` - List of all jobs that were fetched from the EVE API
/// - `startable_jobs` - List of all jobs that can be started in the application
/// - `finished_jobs` - List of jobs that are already done
/// - `container_names` - List of container names
/// - `used_ids` - List application job ids, that were already visited
/// - `used_job_ids` - List of in-game job ids that were already visited
/// 
pub fn job_detection(
    eve_jobs:        &Vec<IndustryJob>,
    startable_jobs:  &Vec<StartableIndustryJobs>,
    finished_jobs:   &Vec<JobId>,
    container_names: &HashMap<ItemId, String>,
    used_ids:        &mut Vec<Uuid>,
    used_job_ids:    &mut Vec<JobId>,
) -> (HashMap<Uuid, Vec<UpdateJobRequest>>, Vec<UnmatchedJob>) {
    let span = span!(Level::DEBUG, "job_detection");
    let _guard = span.enter();

    let now = chrono::Utc::now().naive_utc();

    // List of entries that need to be updated
    let mut updates: HashMap<Uuid, Vec<UpdateJobRequest>> = HashMap::new();
    // List of all jobs that weren`t done, already in the database or matched to
    // to a project
    let mut unmatched_jobs: Vec<UnmatchedJob> = Vec::new();

    for entry in eve_jobs {
        event!(Level::INFO, "[{}] corporation: {}", entry.job_id, entry.corporation_id.map(|x| x.to_string()).unwrap_or("-/-".into()));
        let end_date = NaiveDateTime::parse_from_str(&entry.end_date, "%Y-%m-%dT%H:%M:%SZ").unwrap();

        // 1. The job is already done, so we can continue
        if finished_jobs.contains(&entry.job_id) {
            event!(Level::INFO, "[{}] job is already finished - continue to next", entry.job_id);
            continue;
        }

        // 2. Try to find already existing jobs tagged with the job_id
        //    we can assume that the status is either Building or Done
        //    - if its Done, we are finished
        //    - if its Building we need to check if the job has changed its
        //      status and update it
        // 
        if let Some(job) = startable_jobs
            .iter()
            .find(|x|
                x.job_id.is_some() &&
                x.job_id.unwrap() == entry.job_id
            ) {
            event!(Level::INFO, "[{}] job is marked as done", entry.job_id);
            // The job changed its status from Building to done
            // TODO: this can probably be replaced by just checking the eve status
            if job.status == ProjectJobStatusDatabase::Building && now > end_date {
                let update = UpdateJobRequest {
                    id:           job.id,
                    character_id: Some(entry.installer_id),
                    project_id:   Some(job.project_id),
                    type_id:      entry.product_type_id,
                    cost:         entry.cost,
                    status:       ProjectJobStatusDatabase::Done,
                    job_id:       Some(*entry.job_id),
                };
                updates
                    .entry(job.project_id)
                    .and_modify(|e: &mut Vec<UpdateJobRequest>| e.push(update.clone()))
                    .or_insert(vec![update]);
                used_ids.push(job.id);
            }
            continue;
        }

        // 3. Try to find the project based on its output location
        if let Some(container) = container_names.get(&ItemId(*entry.output_location_id)) {
            if let Some(job) = startable_jobs
                .iter()
                .find(|x|
                    x.type_id == entry.product_type_id &&
                    x.runs == entry.runs &&
                    x.job_id.is_none() &&
                    x.project_name == *container &&
                    !used_ids.contains(&x.id) &&
                    !used_job_ids.contains(&entry.job_id)
                ) {
                    event!(Level::INFO, "[{}] found a suitable project job", entry.job_id);
                    let update = UpdateJobRequest {
                        id:           job.id,
                        character_id: Some(entry.installer_id),
                        project_id:   Some(job.project_id),
                        type_id:      entry.product_type_id,
                        cost:         entry.cost,
                        status:       ProjectJobStatusDatabase::Building,
                        job_id:       Some(*entry.job_id),
                };

                updates
                    .entry(job.project_id)
                    .and_modify(|e: &mut Vec<UpdateJobRequest>| e.push(update.clone()))
                    .or_insert(vec![update]);
                used_ids.push(job.id);
                used_job_ids.push(entry.job_id);
            } else {
                event!(Level::INFO, "[{}] could not find a matching job, marking as unmatched", entry.job_id);
                let project_id = startable_jobs
                    .iter()
                    .find(|x| x.project_name == *container)
                    .map(|x| x.id);

                unmatched_jobs.push(UnmatchedJob {
                    job:        entry.clone(),
                    reason:     UnmatchedJobReason::NoJobFound,
                    project_id: project_id,
                });
            }
        } else {
            event!(Level::INFO, "[{}] job could not be matched", entry.job_id);
            unmatched_jobs.push(UnmatchedJob {
                job:        entry.clone(),
                reason:     UnmatchedJobReason::NoContainer,
                project_id: None,
            });
        }
    }

    (updates, unmatched_jobs)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UnmatchedJob {
    pub job:        IndustryJob,
    pub reason:     UnmatchedJobReason,
    pub project_id: Option<Uuid>,
}

#[derive(
    Clone, Debug, Copy, Hash,
    PartialEq, Eq, PartialOrd, Ord,
    Deserialize, Serialize,
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UnmatchedJobReason {
    /// Output container not found
    NoContainer,
    /// No matching job was found
    NoJobFound,
}

impl UnmatchedJobReason {
    pub fn into_string(self) -> String {
        match self {
            Self::NoContainer => "NO_CONTAINER",
            Self::NoJobFound  => "NO_JOB_FOUND",
        }.into()
    }
}

#[cfg(test)]
mod industry_tests {
    use chrono::Utc;
    use starfoundry_lib_eve_gateway::{IndustryActivity, IndustryJob};
    use starfoundry_lib_types::{CorporationId, ItemId, JobId, LocationId, TypeId};
    use std::{str::FromStr, collections::HashMap};
    use uuid::Uuid;

    use super::{job_detection, ProjectJobStatusDatabase};
    use crate::jobs::{StartableIndustryJobs, UnmatchedJobReason};

    const DEFAULT_CORPORATION: CorporationId = CorporationId(0);

    /// - One project "0"
    /// - Both Jobs are inserted into the project container
    /// - The output container was found
    /// - No unmatched jobs
    #[test]
    fn non_started_all_will_be_started() {
        let project_name = String::from("0");
        let project_id = Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap();

        let db_id_1 = Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap();
        let db_id_2 = Uuid::from_str("00000000-0000-0000-0000-000000000001").unwrap();

        let eve_jobs = vec![
            eve_job(TypeId(0), "2099-01-01T01:01:01Z".into(), 100f32, 1, JobId(1), LocationId(0), DEFAULT_CORPORATION),
            eve_job(TypeId(1), "2099-01-01T01:01:01Z".into(), 100f32, 1, JobId(2), LocationId(0), DEFAULT_CORPORATION)
        ];
        let active_jobs = vec![
            active_job(project_name.clone(), project_id, db_id_1, TypeId(0), 1, ProjectJobStatusDatabase::WaitingForMaterials, None),
            active_job(project_name, project_id, db_id_2, TypeId(1), 1, ProjectJobStatusDatabase::WaitingForMaterials, None),
        ];
        let mut used_ids = Vec::new();
        let mut used_job_ids = Vec::new();

        let mut container_names = HashMap::new();
        container_names.insert(ItemId(0), "0".into());

        let detected_jobs = job_detection(
            &eve_jobs,
            &active_jobs,
            &Vec::new(),
            &container_names,
            &mut used_ids,
            &mut used_job_ids,
        );

        assert_eq!(used_ids.len(), 2);
        assert_eq!(detected_jobs.0.len(), 1);
        assert_eq!(detected_jobs.0.get(&project_id).unwrap().len(), 2);
        assert_eq!(detected_jobs.0.get(&project_id).unwrap()[0].status, ProjectJobStatusDatabase::Building);
        assert_eq!(detected_jobs.0.get(&project_id).unwrap()[1].status, ProjectJobStatusDatabase::Building);
    }

    /// - One project "0"
    /// - 1. job is inserted into the correct output location
    /// - 2. job is inserted into a different invalid output location
    /// - Only the first output location was found
    /// - One unmatched job -> NoContainer
    #[test]
    fn one_will_be_started_one_will_be_without_container() {
        let project_name = String::from("0");
        let project_id = Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap();

        let db_id_1 = Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap();
        let db_id_2 = Uuid::from_str("00000000-0000-0000-0000-000000000001").unwrap();

        let eve_jobs = vec![
            eve_job(TypeId(0), "2099-01-01T01:01:01Z".into(), 100f32, 1, JobId(1), LocationId(0), DEFAULT_CORPORATION),
            eve_job(TypeId(1), "2099-01-01T01:01:01Z".into(), 100f32, 1, JobId(2), LocationId(1), DEFAULT_CORPORATION)
        ];
        let active_jobs = vec![
            active_job(project_name.clone(), project_id, db_id_1, TypeId(0), 1, ProjectJobStatusDatabase::WaitingForMaterials, None),
            active_job(project_name, project_id, db_id_2, TypeId(1), 1, ProjectJobStatusDatabase::WaitingForMaterials, None),
        ];
        let mut used_ids = Vec::new();
        let mut used_job_ids = Vec::new();

        let mut container_names = HashMap::new();
        container_names.insert(ItemId(0), "0".into());

        let detected_jobs = job_detection(
            &eve_jobs,
            &active_jobs,
            &Vec::new(),
            &container_names,
            &mut used_ids,
            &mut used_job_ids,
        );

        assert_eq!(used_ids.len(), 1);
        assert_eq!(detected_jobs.0.len(), 1);
        assert_eq!(detected_jobs.0.get(&project_id).unwrap().len(), 1);
        assert_eq!(detected_jobs.0.get(&project_id).unwrap()[0].status, ProjectJobStatusDatabase::Building);
        assert_eq!(detected_jobs.1.len(), 1);
        assert_eq!(detected_jobs.1[0].reason, UnmatchedJobReason::NoContainer);
    }

    /// - One project "0"
    /// - 1. job is inserted into the correct output location
    /// - 2. job is inserted into the correct output location, but without a
    ///      matching job
    /// - One unmatched job -> NoJobFound
    #[test]
    fn one_will_be_started_one_will_be_without_job() {
        let project_name = String::from("0");
        let project_id = Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap();

        let db_id_1 = Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap();

        let eve_jobs = vec![
            eve_job(TypeId(0), "2099-01-01T01:01:01Z".into(), 100f32, 1, JobId(0), LocationId(0), DEFAULT_CORPORATION),
            eve_job(TypeId(0), "2099-01-01T01:01:01Z".into(), 100f32, 1, JobId(1), LocationId(0), DEFAULT_CORPORATION),
        ];
        let active_jobs = vec![
            active_job(project_name, project_id, db_id_1, TypeId(0), 1, ProjectJobStatusDatabase::WaitingForMaterials, None),
        ];
        let mut used_ids = Vec::new();
        let mut used_job_ids = Vec::new();

        let mut container_names = HashMap::new();
        container_names.insert(ItemId(0), "0".into());

        let detected_jobs = job_detection(
            &eve_jobs,
            &active_jobs,
            &Vec::new(),
            &container_names,
            &mut used_ids,
            &mut used_job_ids,
        );

        assert_eq!(used_ids.len(), 1);
        assert_eq!(detected_jobs.0.len(), 1);
        assert_eq!(detected_jobs.0.get(&project_id).unwrap().len(), 1);
        assert_eq!(detected_jobs.0.get(&project_id).unwrap()[0].status, ProjectJobStatusDatabase::Building);
        assert_eq!(detected_jobs.1.len(), 1);
        assert_eq!(detected_jobs.1[0].reason, UnmatchedJobReason::NoJobFound);
    }

    /// - One project "0"
    /// - 1. job is already in progress
    /// - 2. job is started
    /// - Output location is for both the same and valid
    #[test]
    fn one_started_one_not_all_will_started() {
        let project_name = String::from("0");

        let project_id = Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap();

        let db_id_1 = Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap();
        let db_id_2 = Uuid::from_str("00000000-0000-0000-0000-000000000001").unwrap();

        let eve_jobs = vec![
            eve_job(TypeId(0), "2099-01-01T01:01:01Z".into(), 100f32, 1, JobId(1), LocationId(0), DEFAULT_CORPORATION),
            eve_job(TypeId(1), "2099-01-01T01:01:01Z".into(), 100f32, 1, JobId(2), LocationId(0), DEFAULT_CORPORATION)
        ];
        let active_jobs = vec![
            active_job(project_name.clone(), project_id, db_id_1, TypeId(0), 1, ProjectJobStatusDatabase::Building, Some(JobId(1))),
            active_job(project_name, project_id, db_id_2, TypeId(1), 1, ProjectJobStatusDatabase::WaitingForMaterials, None),
        ];
        let mut used_ids = Vec::new();
        let mut used_job_ids = Vec::new();

        let mut container_names = HashMap::new();
        container_names.insert(ItemId(0), "0".into());

        let detected_jobs = job_detection(
            &eve_jobs,
            &active_jobs,
            &Vec::new(),
            &container_names,
            &mut used_ids,
            &mut used_job_ids,
        );

        assert_eq!(used_job_ids.len(), 1);
        assert_eq!(detected_jobs.0.len(), 1);
        assert_eq!(detected_jobs.0.get(&project_id).unwrap().len(), 1);
        assert_eq!(detected_jobs.0.get(&project_id).unwrap()[0].status, ProjectJobStatusDatabase::Building);
    }

    /// - Two projects "0", "1"
    /// - One EVE job
    /// - Two project jobs
    /// - The EVE job is assigned
    /// - Valid container
    #[test]
    fn two_projects_two_open_jobs_one_will_be_started() {
        let project_name_1 = String::from("0");
        let project_name_2 = String::from("1");

        let project_id_1 = Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap();
        let project_id_2 = Uuid::from_str("00000000-0000-0000-0000-000000000001").unwrap();

        let db_id_1 = Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap();
        let db_id_2 = Uuid::from_str("00000000-0000-0000-0000-000000000001").unwrap();

        let eve_jobs = vec![
            eve_job(TypeId(0), "2099-01-01T01:01:01Z".into(), 100f32, 1, JobId(1), LocationId(0), DEFAULT_CORPORATION),
        ];
        let active_jobs = vec![
            active_job(project_name_1, project_id_1, db_id_1, TypeId(0), 1, ProjectJobStatusDatabase::WaitingForMaterials, None),
            active_job(project_name_2, project_id_2, db_id_2, TypeId(0), 1, ProjectJobStatusDatabase::WaitingForMaterials, None),
        ];
        let mut used_ids = Vec::new();
        let mut used_job_ids = Vec::new();

        let mut container_names = HashMap::new();
        container_names.insert(ItemId(0), "0".into());

        let detected_jobs = job_detection(
            &eve_jobs,
            &active_jobs,
            &Vec::new(),
            &container_names,
            &mut used_ids,
            &mut used_job_ids,
        );

        assert_eq!(used_job_ids.len(), 1);
        assert_eq!(detected_jobs.0.len(), 1);
        assert_eq!(detected_jobs.0.get(&project_id_1).unwrap().len(), 1);
        assert_eq!(detected_jobs.0.get(&project_id_1).unwrap()[0].status, ProjectJobStatusDatabase::Building);
    }

    /// - Two projects "0", "1"
    /// - Two EVE jobs
    /// - Two project jobs
    /// - One Job already running and assigned
    /// - The EVE job is assigned
    /// - Valid container
    #[test]
    fn two_projects_one_job_started_other_will_be_started() {
        let project_name_1 = String::from("0");
        let project_name_2 = String::from("1");

        let project_id_1 = Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap();
        let project_id_2 = Uuid::from_str("00000000-0000-0000-0000-000000000001").unwrap();

        let db_id_1 = Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap();
        let db_id_2 = Uuid::from_str("00000000-0000-0000-0000-000000000001").unwrap();

        let eve_jobs = vec![
            eve_job(TypeId(0), "2099-01-01T01:01:01Z".into(), 100f32, 1, JobId(0), LocationId(0), DEFAULT_CORPORATION),
            eve_job(TypeId(0), "2099-01-01T01:01:01Z".into(), 100f32, 1, JobId(1), LocationId(1), DEFAULT_CORPORATION),
        ];
        let active_jobs = vec![
            active_job(project_name_1, project_id_1, db_id_1, TypeId(0), 1, ProjectJobStatusDatabase::Building, Some(JobId(0))),
            active_job(project_name_2, project_id_2, db_id_2, TypeId(0), 1, ProjectJobStatusDatabase::WaitingForMaterials, None),
        ];
        let mut used_ids = Vec::new();
        let mut used_job_ids = Vec::new();

        let mut container_names = HashMap::new();
        container_names.insert(ItemId(0), "0".into());
        container_names.insert(ItemId(1), "1".into());

        let detected_jobs = job_detection(
            &eve_jobs,
            &active_jobs,
            &Vec::new(),
            &container_names,
            &mut used_ids,
            &mut used_job_ids,
        );

        assert_eq!(used_job_ids.len(), 1);
        assert_eq!(detected_jobs.0.len(), 1);
        assert_eq!(detected_jobs.0.get(&project_id_2).unwrap().len(), 1);
        assert_eq!(detected_jobs.0.get(&project_id_2).unwrap()[0].status, ProjectJobStatusDatabase::Building);
    }

    /// - Two projects "0", "1"
    /// - One EVE job
    /// - One Job already running and assigned
    /// - The EVE job is assigned
    /// - Valid container
    /// - Multiple rounds of detection
    #[test]
    fn two_projects_two_jobs_one_should_be_started_the_other_ignored() {
        let project_name_1 = String::from("0");
        let project_name_2 = String::from("1");

        let project_id_1 = Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap();
        let project_id_2 = Uuid::from_str("00000000-0000-0000-0000-000000000001").unwrap();

        let db_id_1 = Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap();
        let db_id_2 = Uuid::from_str("00000000-0000-0000-0000-000000000001").unwrap();

        let eve_jobs = vec![
            eve_job(TypeId(0), "2099-01-01T01:01:01Z".into(), 100f32, 1, JobId(0), LocationId(0), DEFAULT_CORPORATION),
        ];
        let active_jobs = vec![
            active_job(project_name_1, project_id_1, db_id_1, TypeId(0), 1, ProjectJobStatusDatabase::WaitingForMaterials, None),
            active_job(project_name_2, project_id_2, db_id_2, TypeId(0), 1, ProjectJobStatusDatabase::WaitingForMaterials, None),
        ];
        let mut used_ids = Vec::new();
        let mut used_job_ids = Vec::new();

        let mut container_names = HashMap::new();
        container_names.insert(ItemId(0), "0".into());

        let detected_jobs_1 = job_detection(
            &eve_jobs,
            &active_jobs,
            &Vec::new(),
            &container_names,
            &mut used_ids,
            &mut used_job_ids,
        );
        let detected_jobs_2 = job_detection(
            &eve_jobs,
            &active_jobs,
            &Vec::new(),
            &container_names,
            &mut used_ids,
            &mut used_job_ids,
        );

        assert_eq!(used_job_ids.len(), 1);
        assert_eq!(detected_jobs_1.0.len(), 1);
        assert_eq!(detected_jobs_1.0.get(&project_id_1).unwrap().len(), 1);
        assert_eq!(detected_jobs_1.0.get(&project_id_1).unwrap()[0].status, ProjectJobStatusDatabase::Building);
        assert_eq!(used_job_ids[0], JobId(0));
        assert_eq!(detected_jobs_2.0.len(), 0);
        assert!(detected_jobs_1.0.get(&project_id_2).is_none());
        assert!(detected_jobs_2.0.get(&project_id_2).is_none());
    }

    #[test]
    fn one_matching_run_one_non_matching_run() {
        let project_name = String::from("0");

        let project_id = Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap();

        let db_id_1 = Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap();
        let db_id_2 = Uuid::from_str("00000000-0000-0000-0000-000000000001").unwrap();

        let eve_jobs = vec![
            eve_job(TypeId(0), "2099-01-01T01:01:01Z".into(), 100f32, 2, JobId(1), LocationId(0), DEFAULT_CORPORATION),
            eve_job(TypeId(1), "2099-01-01T01:01:01Z".into(), 100f32, 1, JobId(2), LocationId(0), DEFAULT_CORPORATION),
        ];
        let active_jobs = vec![
            active_job(project_name.clone(), project_id, db_id_1, TypeId(0), 1, ProjectJobStatusDatabase::WaitingForMaterials, None),
            active_job(project_name, project_id, db_id_2, TypeId(1), 1, ProjectJobStatusDatabase::WaitingForMaterials, None),
        ];
        let mut used_ids = Vec::new();
        let mut used_job_ids = Vec::new();

        let mut container_names = HashMap::new();
        container_names.insert(ItemId(0), "0".into());

        let detected_jobs = job_detection(
            &eve_jobs,
            &active_jobs,
            &Vec::new(),
            &container_names,
            &mut used_ids,
            &mut used_job_ids,
        );

        assert_eq!(used_ids.len(), 1);
        assert_eq!(detected_jobs.0.len(), 1);
        assert_eq!(detected_jobs.0.get(&project_id).unwrap().len(), 1);
        assert_eq!(detected_jobs.0.get(&project_id).unwrap()[0].status, ProjectJobStatusDatabase::Building);
        assert_eq!(detected_jobs.1.len(), 1);
    }

    fn eve_job(
        type_id:        TypeId,
        end_date:       String,
        cost:           f32,
        runs:           i32,
        job_id:         JobId,
        location_id:    LocationId,
        corporation_id: CorporationId,
    ) -> IndustryJob {
        IndustryJob {
            activity:              IndustryActivity::Reactions,
            blueprint_id:          ItemId(0),
            blueprint_type_id:     0.into(),
            blueprint_location_id: LocationId(0),
            product_type_id:       type_id,
            end_date:              end_date,
            cost:                  Some(cost),
            runs:                  runs,
            licensed_runs:         0,
            job_id:                job_id,
            facility_id:           0i64,
            installer_id:          0.into(),
            status:                "".into(),
            corporation_id:        Some(corporation_id),
            location_id:           location_id,
            output_location_id:    location_id,
        }
    }

    fn active_job(
        project_name: String,
        project_id:   Uuid,
        id:           Uuid,
        type_id:      TypeId,
        runs:         i32,
        status:       ProjectJobStatusDatabase,
        job_id:       Option<JobId>,
    ) -> StartableIndustryJobs {
        StartableIndustryJobs {
            project_name,
            project_id,
            id,
            type_id,
            runs,
            status,
            job_id,
            created_at: Utc::now(),
        }
    }
}
