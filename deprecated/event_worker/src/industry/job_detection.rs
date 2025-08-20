use chrono::NaiveDateTime;
use starfoundry_lib_eve_api::IndustryJobEntry;
use starfoundry_lib_projects::ProjectJobStatus;
use starfoundry_lib_types::JobId;
use std::collections::HashMap;
use uuid::Uuid;

use super::{StartableIndustryJobs, UpdateJobRequest};

/// Attempts to match an industry job retrieved from the eve api, to a industry
/// job that needs to be done in order to complete a project.
/// 
pub fn job_detection(
    eve_jobs:        &Vec<IndustryJobEntry>,
    startable_jobs:  &Vec<StartableIndustryJobs>,
    finished_jobs:   &Vec<JobId>,
    ignored_jobs:    &Vec<JobId>,
    container_names: &HashMap<i64, String>,
    used_ids:        &mut Vec<Uuid>,
    used_job_ids:    &mut Vec<JobId>,
) -> (HashMap<Uuid, Vec<UpdateJobRequest>>, Vec<IndustryJobEntry>) {
    let now = chrono::Utc::now().naive_utc();

    // List of entries that need to be updated
    let mut updates: HashMap<Uuid, Vec<UpdateJobRequest>> = HashMap::new();
    // List of all jobs that weren`t done, already in the database or matched to
    // to a project
    let mut unmatched_jobs = Vec::new();

    for entry in eve_jobs {
        let end_date = NaiveDateTime::parse_from_str(&entry.end_date, "%Y-%m-%dT%H:%M:%SZ").unwrap();

        // 1. The job is already done, so we can continue
        if finished_jobs.contains(&entry.job_id) {
            continue;
        }

        // 2. the job is ignored, so we can continue
        if ignored_jobs.contains(&entry.job_id) {
            continue;
        }

        // 3. Try to find already existing jobs tagged with the job_id
        //    we can assume that the status is either Building or Done
        //    - if its Done, we are finsihed
        //    - if its Building we need to check if the job has changed its
        //      status and update it
        // 
        if let Some(job) = startable_jobs
            .iter()
            .find(|x|
                x.job_id.is_some() &&
                x.job_id.unwrap() == entry.job_id
            ) {
            // The job changed its status from Building to done
            // TODO: this can probably be replaced by just checking the eve status
            if job.status == ProjectJobStatus::Building && now > end_date {
                let update = UpdateJobRequest {
                    id:           job.id,
                    character_id: Some(entry.installer_id),
                    project_id:   Some(job.project_id),
                    type_id:      entry.product_type_id,
                    cost:         entry.cost,
                    status:       ProjectJobStatus::Done,
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

        // 4. Try to find the project based on its output location
        if let Some(container) = container_names.get(&*entry.output_location_id) {
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
                    let update = UpdateJobRequest {
                        id:           job.id,
                        character_id: Some(entry.installer_id),
                        project_id:   Some(job.project_id),
                        type_id:      entry.product_type_id,
                        cost:         entry.cost,
                        status:       ProjectJobStatus::Building,
                        job_id:       Some(*entry.job_id),
                };

                updates
                    .entry(job.project_id)
                    .and_modify(|e: &mut Vec<UpdateJobRequest>| e.push(update.clone()))
                    .or_insert(vec![update]);
                used_ids.push(job.id);
                used_job_ids.push(entry.job_id);
            } else {
                unmatched_jobs.push(entry);
            }
        } else {
            unmatched_jobs.push(entry);
        }
    }

    // 5. Try to find an entry that matches our current job
    // 
    // If we don´t find anything after step one, two, and three then there
    // is no matching event.
    // We also prevent that one job is assigned to multiple jobs, by 
    // filtering in step 1 on the job id
    //
    for unmatched in unmatched_jobs.clone() {
        if let Some(job) = startable_jobs
            .iter()
            .find(|x|
                x.type_id == unmatched.product_type_id &&
                x.runs == unmatched.runs &&
                x.job_id.is_none() &&
                !used_ids.contains(&x.id) &&
                !used_job_ids.contains(&unmatched.job_id)
            ) {
            let update = UpdateJobRequest {
                id:           job.id,
                character_id: Some(unmatched.installer_id),
                project_id:   Some(job.project_id),
                type_id:      unmatched.product_type_id,
                cost:         unmatched.cost,
                status:       ProjectJobStatus::Building,
                job_id:       Some(*unmatched.job_id),
            };

            updates
                .entry(job.project_id)
                .and_modify(|e: &mut Vec<UpdateJobRequest>| e.push(update.clone()))
                .or_insert(vec![update]);

            used_ids.push(job.id);
            used_job_ids.push(unmatched.job_id);
            unmatched_jobs.pop();
        }
    }

    let unmatched_jobs = unmatched_jobs
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    (updates, unmatched_jobs)
}

#[cfg(test)]
mod industry_tests {
    use chrono::Utc;
    use starfoundry_lib_eve_api::{IndustryJobEntry, IndustryActivity};
    use starfoundry_lib_types::{JobId, TypeId, LocationId, ItemId, CorporationId};
    use std::{str::FromStr, collections::HashMap};
    use uuid::Uuid;

    use super::{job_detection, ProjectJobStatus};
    use crate::industry::StartableIndustryJobs;

    const DEFAULT_CORPORATION: CorporationId = CorporationId(0);

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
            active_job(project_name.clone(), project_id, db_id_1, TypeId(0), 1, ProjectJobStatus::WaitingForMaterials, None),
            active_job(project_name, project_id, db_id_2, TypeId(1), 1, ProjectJobStatus::WaitingForMaterials, None),
        ];
        let mut used_ids = Vec::new();
        let mut used_job_ids = Vec::new();

        let detected_jobs = job_detection(
            &eve_jobs,
            &active_jobs,
            &Vec::new(),
            &Vec::new(),
            &HashMap::new(),
            &mut used_ids,
            &mut used_job_ids,
        );

        assert_eq!(used_ids.len(), 2);
        assert_eq!(detected_jobs.0.len(), 1);
        assert_eq!(detected_jobs.0.get(&project_id).unwrap().len(), 2);
        assert_eq!(detected_jobs.0.get(&project_id).unwrap()[0].status, ProjectJobStatus::Building);
        assert_eq!(detected_jobs.0.get(&project_id).unwrap()[1].status, ProjectJobStatus::Building);
    }

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
            active_job(project_name.clone(), project_id, db_id_1, TypeId(0), 1, ProjectJobStatus::Building, Some(JobId(1))),
            active_job(project_name, project_id, db_id_2, TypeId(1), 1, ProjectJobStatus::WaitingForMaterials, None),
        ];
        let mut used_ids = Vec::new();
        let mut used_job_ids = Vec::new();

        let detected_jobs = job_detection(
            &eve_jobs,
            &active_jobs,
            &Vec::new(),
            &Vec::new(),
            &HashMap::new(),
            &mut used_ids,
            &mut used_job_ids,
        );

        assert_eq!(used_job_ids.len(), 1);
        assert_eq!(detected_jobs.0.len(), 1);
        assert_eq!(detected_jobs.0.get(&project_id).unwrap().len(), 1);
        assert_eq!(detected_jobs.0.get(&project_id).unwrap()[0].status, ProjectJobStatus::Building);
    }

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
            active_job(project_name_1, project_id_1, db_id_1, TypeId(0), 1, ProjectJobStatus::WaitingForMaterials, None),
            active_job(project_name_2, project_id_2, db_id_2, TypeId(0), 1, ProjectJobStatus::WaitingForMaterials, None),
        ];
        let mut used_ids = Vec::new();
        let mut used_job_ids = Vec::new();

        let detected_jobs = job_detection(
            &eve_jobs,
            &active_jobs,
            &Vec::new(),
            &Vec::new(),
            &HashMap::new(),
            &mut used_ids,
            &mut used_job_ids,
        );

        assert_eq!(used_job_ids.len(), 1);
        assert_eq!(detected_jobs.0.len(), 1);
        assert_eq!(detected_jobs.0.get(&project_id_1).unwrap().len(), 1);
        assert_eq!(detected_jobs.0.get(&project_id_1).unwrap()[0].status, ProjectJobStatus::Building);
    }

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
            eve_job(TypeId(0), "2099-01-01T01:01:01Z".into(), 100f32, 1, JobId(1), LocationId(0), DEFAULT_CORPORATION),
        ];
        let active_jobs = vec![
            active_job(project_name_1, project_id_1, db_id_1, TypeId(0), 1, ProjectJobStatus::Building, Some(JobId(0))),
            active_job(project_name_2, project_id_2, db_id_2, TypeId(0), 1, ProjectJobStatus::WaitingForMaterials, None),
        ];
        let mut used_ids = Vec::new();
        let mut used_job_ids = Vec::new();

        let detected_jobs = job_detection(
            &eve_jobs,
            &active_jobs,
            &Vec::new(),
            &Vec::new(),
            &HashMap::new(),
            &mut used_ids,
            &mut used_job_ids,
        );

        assert_eq!(used_job_ids.len(), 1);
        assert_eq!(detected_jobs.0.len(), 1);
        assert_eq!(detected_jobs.0.get(&project_id_2).unwrap().len(), 1);
        assert_eq!(detected_jobs.0.get(&project_id_2).unwrap()[0].status, ProjectJobStatus::Building);
    }

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
            active_job(project_name_1, project_id_1, db_id_1, TypeId(0), 1, ProjectJobStatus::WaitingForMaterials, None),
            active_job(project_name_2, project_id_2, db_id_2, TypeId(0), 1, ProjectJobStatus::WaitingForMaterials, None),
        ];
        let mut used_ids = Vec::new();
        let mut used_job_ids = Vec::new();

        let detected_jobs_1 = job_detection(
            &eve_jobs,
            &active_jobs,
            &Vec::new(),
            &Vec::new(),
            &HashMap::new(),
            &mut used_ids,
            &mut used_job_ids,
        );
        let detected_jobs_2 = job_detection(
            &eve_jobs,
            &active_jobs,
            &Vec::new(),
            &Vec::new(),
            &HashMap::new(),
            &mut used_ids,
            &mut used_job_ids,
        );

        assert_eq!(used_job_ids.len(), 1);
        assert_eq!(detected_jobs_1.0.len(), 1);
        assert_eq!(detected_jobs_1.0.get(&project_id_1).unwrap().len(), 1);
        assert_eq!(detected_jobs_1.0.get(&project_id_1).unwrap()[0].status, ProjectJobStatus::Building);
        assert_eq!(used_job_ids[0], JobId(0));
        assert_eq!(detected_jobs_2.0.len(), 0);
        assert!(detected_jobs_1.0.get(&project_id_2).is_none());
        assert!(detected_jobs_2.0.get(&project_id_2).is_none());
    }

    // JobId(0) will match to project_2, because the LocationId(0) is mapped to
    // project_2
    // project_2 is 00000000-0000-0000-0000-000000000001
    //
    // JobId(1) will be matched to project_1 as it will run through the
    // optimistic job matching routine
    #[test]
    fn non_started_project_two_will_be_started_because_of_container() {
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
            active_job(project_name_1, project_id_1, db_id_1, TypeId(0), 1, ProjectJobStatus::WaitingForMaterials, None),
            active_job(project_name_2.clone(), project_id_2, db_id_2, TypeId(0), 1, ProjectJobStatus::WaitingForMaterials, None),
        ];
        let mut container_names: HashMap<i64, String> = HashMap::new();
        container_names.insert(0, project_name_2);

        let mut used_ids = Vec::new();
        let mut used_job_ids = Vec::new();

        let detected_jobs = job_detection(
            &eve_jobs,
            &active_jobs,
            &Vec::new(),
            &Vec::new(),
            &container_names,
            &mut used_ids,
            &mut used_job_ids,
        );

        assert_eq!(used_job_ids.len(), 2);
        assert_eq!(detected_jobs.0.len(), 2);
        assert_eq!(detected_jobs.0.get(&project_id_1).unwrap().len(), 1);
        assert_eq!(detected_jobs.0.get(&project_id_1).unwrap()[0].status, ProjectJobStatus::Building);
        assert_eq!(detected_jobs.0.get(&project_id_1).unwrap()[0].id, Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap());

        assert_eq!(detected_jobs.0.get(&project_id_2).unwrap().len(), 1);
        assert_eq!(detected_jobs.0.get(&project_id_2).unwrap()[0].status, ProjectJobStatus::Building);
        assert_eq!(detected_jobs.0.get(&project_id_2).unwrap()[0].id, Uuid::from_str("00000000-0000-0000-0000-000000000001").unwrap());
    }

    #[test]
    fn non_started_no_matching_job_exists_at_project() {
        let project_name_1 = String::from("0");
        let project_name_2 = String::from("1");

        let project_id_1 = Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap();
        let project_id_2 = Uuid::from_str("00000000-0000-0000-0000-000000000001").unwrap();

        let db_id_1 = Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap();
        let db_id_2 = Uuid::from_str("00000000-0000-0000-0000-000000000001").unwrap();
        let db_id_3 = Uuid::from_str("00000000-0000-0000-0000-000000000002").unwrap();

        let eve_jobs = vec![
            eve_job(TypeId(0), "2099-01-01T01:01:01Z".into(), 100f32, 2, JobId(0), LocationId(0), DEFAULT_CORPORATION),
        ];
        let active_jobs = vec![
            active_job(project_name_1.clone(), project_id_1, db_id_1, TypeId(0), 1, ProjectJobStatus::WaitingForMaterials, None),
            active_job(project_name_1.clone(), project_id_1, db_id_2, TypeId(0), 1, ProjectJobStatus::WaitingForMaterials, None),
            active_job(project_name_2, project_id_2, db_id_3, TypeId(0), 2, ProjectJobStatus::WaitingForMaterials, None),
        ];
        // Assign the LocationId(0) to project_1 which has no such job
        let mut container_names: HashMap<i64, String> = HashMap::new();
        container_names.insert(0, project_name_1);

        let mut used_ids = Vec::new();
        let mut used_job_ids = Vec::new();

        let detected_jobs = job_detection(
            &eve_jobs,
            &active_jobs,
            &Vec::new(),
            &Vec::new(),
            &container_names,
            &mut used_ids,
            &mut used_job_ids,
        );

        assert_eq!(used_job_ids.len(), 1);
        assert_eq!(detected_jobs.0.len(), 1);
        assert_eq!(detected_jobs.0.get(&project_id_1).is_none(), true);

        assert_eq!(detected_jobs.0.get(&project_id_2).unwrap().len(), 1);
        assert_eq!(detected_jobs.0.get(&project_id_2).unwrap()[0].status, ProjectJobStatus::Building);
        assert_eq!(detected_jobs.0.get(&project_id_2).unwrap()[0].id, Uuid::from_str("00000000-0000-0000-0000-000000000002").unwrap());
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
            active_job(project_name.clone(), project_id, db_id_1, TypeId(0), 1, ProjectJobStatus::WaitingForMaterials, None),
            active_job(project_name, project_id, db_id_2, TypeId(1), 1, ProjectJobStatus::WaitingForMaterials, None),
        ];
        let mut used_ids = Vec::new();
        let mut used_job_ids = Vec::new();

        let detected_jobs = job_detection(
            &eve_jobs,
            &active_jobs,
            &Vec::new(),
            &Vec::new(),
            &HashMap::new(),
            &mut used_ids,
            &mut used_job_ids,
        );

        assert_eq!(used_ids.len(), 1);
        assert_eq!(detected_jobs.0.len(), 1);
        assert_eq!(detected_jobs.0.get(&project_id).unwrap().len(), 1);
        assert_eq!(detected_jobs.0.get(&project_id).unwrap()[0].status, ProjectJobStatus::Building);
        assert_eq!(detected_jobs.1.len(), 1);
    }

    #[test]
    fn ignore_job() {
        let project_name = String::from("0");

        let project_id = Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap();

        let db_id_1 = Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap();
        let db_id_2 = Uuid::from_str("00000000-0000-0000-0000-000000000001").unwrap();

        let eve_jobs = vec![
            eve_job(TypeId(0), "2099-01-01T01:01:01Z".into(), 100f32, 1, JobId(1), LocationId(0), DEFAULT_CORPORATION),
            eve_job(TypeId(1), "2099-01-01T01:01:01Z".into(), 100f32, 1, JobId(2), LocationId(0), DEFAULT_CORPORATION)
        ];
        let active_jobs = vec![
            active_job(project_name.clone(), project_id, db_id_1, TypeId(0), 1, ProjectJobStatus::WaitingForMaterials, None),
            active_job(project_name, project_id, db_id_2, TypeId(1), 1, ProjectJobStatus::WaitingForMaterials, None),
        ];
        let mut used_ids = Vec::new();
        let mut used_job_ids = Vec::new();

        let detected_jobs = job_detection(
            &eve_jobs,
            &active_jobs,
            &Vec::new(),
            &vec![
                JobId(2)
            ],
            &HashMap::new(),
            &mut used_ids,
            &mut used_job_ids,
        );

        assert_eq!(used_ids.len(), 1);
        assert_eq!(detected_jobs.0.len(), 1);
        assert_eq!(detected_jobs.0.get(&project_id).unwrap().len(), 1);
        assert_eq!(detected_jobs.0.get(&project_id).unwrap()[0].status, ProjectJobStatus::Building);
    }

    fn eve_job(
        type_id:        TypeId,
        end_date:       String,
        cost:           f32,
        runs:           i32,
        job_id:         JobId,
        location_id:    LocationId,
        corporation_id: CorporationId,
    ) -> IndustryJobEntry {
        IndustryJobEntry {
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
        status:       ProjectJobStatus,
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
