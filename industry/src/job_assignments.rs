mod create_job_assignment;
mod error;
mod list_job_assignments;
mod update_job_assignment;
mod service;

use starfoundry_lib_types::starfoundry_uuid;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::AppState;

// TODO: check write permission
pub fn routes() -> OpenApiRouter<AppState> {
    let create_job_assignment = OpenApiRouter::new()
        .routes(routes!(create_job_assignment::api));

    let list_job_assignments = OpenApiRouter::new()
        .routes(routes!(list_job_assignments::api));

    let update_job_assignment = OpenApiRouter::new()
        .routes(routes!(update_job_assignment::api));

    OpenApiRouter::new()
        .merge(create_job_assignment)
        .merge(list_job_assignments)
        .merge(update_job_assignment)
}

starfoundry_uuid!(JobAssignmentUuid, "JobAssignmentUuid");
