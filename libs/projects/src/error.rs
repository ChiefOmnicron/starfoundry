use starfoundry_lib_structures::StructureUuid;
use starfoundry_lib_types::{CharacterId, JobId, TypeId};
use thiserror::Error;
use uuid::Uuid;

use crate::{ProjectFilter, ProjectGroupUuid, ProjectJobAssignmentUuid, ProjectJobUuid, ProjectMarketUuid, ProjectMiscUuid, ProjectUuid};

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    #[error("Not eistonen")]
    NotEistonen,
    #[error("Validating the input data failed, '{0}'")]
    ValidationError(String),

    #[error("general appraisal error, error: '{0}'")]
    AppraisalError(starfoundry_lib_appraisal::Error),
    #[error("no project with id '{0}'")]
    ProjectNotFound(ProjectUuid),
    #[error("no project group with id '{0}'")]
    ProjectGroupNotFound(ProjectGroupUuid),
    #[error("the character '{1}' is not allowed to accedd '{0}'")]
    Forbidden(Uuid, CharacterId),
    #[error("error while fetching permissions for project '{1}', error: '{0}'")]
    FetchPermissions(sqlx::Error, ProjectUuid),

    // root
    #[error("error while creating project, error: '{0}'")]
    CreateProject(sqlx::Error),
    #[error("error while deleteing project '{1}', error: '{0}'")]
    DeleteProject(sqlx::Error, ProjectUuid),
    #[error("error while fetching project '{1}', error: '{0}'")]
    FetchProject(sqlx::Error, ProjectUuid),
    #[error("error while fetching project products for '{1}', error: '{0}'")]
    FetchProducts(sqlx::Error, ProjectUuid),
    #[error("error while listing all projects for '{1}' with filter '{2:?}', error: '{0}'")]
    ListProjectIds(sqlx::Error, CharacterId, ProjectFilter),
    #[error("error while updating project '{1}', error: '{0}'")]
    UpdateProject(sqlx::Error, ProjectUuid),

    // excess
    #[error("error while adding excess for project '{1}', error: '{0}'")]
    AddExcess(sqlx::Error, ProjectUuid),
    #[error("error while fetching excess for project '{1}', error: '{0}'")]
    FetchExcess(sqlx::Error, ProjectUuid),
    #[error("error while updating project excess prices for project '{1}', error: '{0}'")]
    UpdateExcessPrice(sqlx::Error, ProjectUuid),

    // finance
    #[error("error while fetching project finances '{1}', error: '{0}'")]
    FetchFinance(sqlx::Error, ProjectUuid),

    // group
    #[error("error while accepting invite to project group '{1}', error: '{0}'")]
    AcceptGroupInvite(sqlx::Error, ProjectGroupUuid),
    #[error("error while accepting member to project group '{1}', error: '{0}'")]
    AcceptGroupMember(sqlx::Error, ProjectGroupUuid),
    #[error("error while creating project group, error: '{0}'")]
    CreateGroup(sqlx::Error),
    #[error("error while deleting project group '{1}', error: '{0}'")]
    DeleteGroup(sqlx::Error, ProjectGroupUuid),
    #[error("error while fetching project '{1}', error: '{0}'")]
    FetchGroup(sqlx::Error, ProjectGroupUuid),
    #[error("error while fetching permissions for project group '{1}', error: '{0}'")]
    FetchGroupPermissions(sqlx::Error, ProjectGroupUuid),
    #[error("error while fetching defaults for project group '{1}', error: '{0}'")]
    FetchGroupDefaults(sqlx::Error, ProjectGroupUuid),
    #[error("error while fetching all project groups, error: '{0}'")]
    ListGroups(sqlx::Error),
    #[error("error while fetching project group members for group '{1}', error: '{0}'")]
    ListGroupMembers(sqlx::Error, ProjectGroupUuid),
    #[error("error while updating project group '{1}', error: '{0}'")]
    UpdateGroup(sqlx::Error, ProjectGroupUuid),
    #[error("error while updating group member '{1}', error: '{0}'")]
    UpdateGroupMember(sqlx::Error, ProjectGroupUuid, CharacterId),
    #[error("error while updating group defaults '{1}', error: '{0}'")]
    UpdateGroupDefaults(sqlx::Error, ProjectGroupUuid),
    #[error("error while removing group member '{1}', error: '{0}'")]
    RemoveGroupMember(sqlx::Error, ProjectGroupUuid, CharacterId),

    // jobs
    #[error("error while fetching jobs for project '{1}', error: '{0}'")]
    FetchJobs(sqlx::Error, ProjectUuid),
    #[error("error while fetching active project jobs for '{1}', error: '{0}'")]
    FetchActiveJobs(sqlx::Error, ProjectUuid),
    #[error("error fetching project jobs '{1:?}', error: '{0}'")]
    FetchJobsByArray(sqlx::Error, Vec<ProjectJobUuid>),
    #[error("error while fetching startable project jobs for '{1}', error: '{0}'")]
    FetchStartableJobs(sqlx::Error, ProjectUuid),
    #[error("error while deleting job '{2}' for project '{1}', error: '{0}'")]
    DeleteJob(sqlx::Error, ProjectUuid, ProjectJobUuid),
    #[error("error while inserting project jobs, '{0}'")]
    InsertJobs(sqlx::Error),
    #[error("error while updating jobs for project '{1}', 'error: {0}'")]
    UpdateJobs(sqlx::Error, ProjectUuid),

    // job detection
    #[error("error while fetching job detection logs, error: '{0}'")]
    FetchJobDetection(sqlx::Error),
    #[error("error while fetching project job by it's JobId '{1}', error: '{0}'")]
    FetchProjectJobByJobId(sqlx::Error, JobId),
    #[error("error while updating project job by it's JobId '{1}', error: '{0}'")]
    UpdateProjectJobByJobId(sqlx::Error, JobId),
    #[error("error while updating industry job by it's JobId '{1}', error: '{0}'")]
    UpdateIndustryJobByJobId(sqlx::Error, JobId),
    #[error("error while updating job detection, error: '{0}'")]
    UpdateJobDetection(sqlx::Error),
    #[error("error while inserting project job, error: '{0}'")]
    InsertProjectJob(sqlx::Error),
    #[error("error while deleting project job, error: '{0}'")]
    DeleteProjectJob(sqlx::Error),
    #[error("error while deleing from project jobs by job id '{1}', error: '{0}'")]
    DeleteFromProjectJobByJobId(sqlx::Error, JobId),

    // product
    #[error("error while adding entries for project '{1}', error: '{0}'")]
    AddProduct(sqlx::Error, ProjectUuid),

    // market
    #[error("error while inserting new market entries for project '{1}', error: '{0}'")]
    AddMarket(sqlx::Error, ProjectUuid),
    #[error("error while fetching market for project '{1}', error: '{0}'")]
    FetchMarket(sqlx::Error, ProjectUuid),
    #[error("error while fetching market prices for project '{1}', error: '{0}'")]
    FetchMarketPrices(sqlx::Error, ProjectUuid),
    #[error("error while deleting market entry '{2}' for project '{1}', error: '{0}'")]
    DeleteMarket(sqlx::Error, ProjectUuid, ProjectMarketUuid),
    #[error("error while deleting market minerals for project '{1}', error: '{0}'")]
    DeleteMarketMinerals(sqlx::Error, ProjectUuid),
    #[error("error while updating market entries for project '{1}', error: '{0}'")]
    UpdateMarket(sqlx::Error, ProjectUuid),
    #[error("error while fetching the last market update")]
    LatestMarketFetch(sqlx::Error, StructureUuid),
    #[error("there was no latest market fetch")]
    NoLatestMarketFetch,

    // misc
    #[error("error while adding misc entry for project '{1}', error: '{0}'")]
    AddMisc(sqlx::Error, ProjectUuid),
    #[error("error while fetching misc for project '{1}', error: '{0}'")]
    FetchMisc(sqlx::Error, ProjectUuid),
    #[error("error while deleting misc entry '{2}' for project '{1}', error: '{0}'")]
    DeleteMisc(sqlx::Error, ProjectUuid, ProjectMiscUuid),
    #[error("error while updating misc entry '{2}' for project '{1}', error: '{0}'")]
    UpdateMisc(sqlx::Error, ProjectUuid, ProjectMiscUuid),

    // stock
    #[error("error while adding stock for project '{1}', error: '{0}'")]
    AddStock(sqlx::Error, ProjectUuid),
    #[error("error while fetching stock for project '{1}', error: '{0}'")]
    FetchStock(sqlx::Error, ProjectUuid),
    #[error("error while updating project stock prices for project '{1}', error: '{0}'")]
    UpdateStockPrice(sqlx::Error, ProjectUuid),

    // job assignments
    #[error("error while creating project job assignment, error: '{0}'")]
    CreateJobAssignment(sqlx::Error),
    #[error("error while fetching project job assigments with id '{1}', error: '{0}'")]
    FetchProjectJobAssignments(sqlx::Error, ProjectJobAssignmentUuid),
    #[error("project job assigments with id '{0}' does not exist")]
    ProjectJobAssignmentNotFound(ProjectJobAssignmentUuid),
    #[error("error while updating project job assignment job, group: '{1}', job: '{2}', error: '{0}'")]
    UpdateProjectJobAssignmentJob(sqlx::Error, ProjectJobAssignmentUuid, ProjectJobUuid),

    #[error("the structure could not be found, '{0}'")]
    StructureNotFound(StructureUuid),
    // TODO: needs moved to starfoundry_lib_structures
    #[error("error while fetching structure ids for group, '{0}'")]
    FetchStructureGroup(sqlx::Error),
    #[error("error while fetching dynamic structure ids for group, '{0}'")]
    FetchDynamicStructureGroup(sqlx::Error),
    #[error("no valid structure group found")]
    NoValidStructureGroup,

    #[error("error while parsing json dependency")]
    ParseJsonToDependency(serde_json::Error),
    #[error("error while fetching blueprint json '{1}', '{0}'")]
    FetchBlueprintJson(sqlx::Error, TypeId),

    #[error("error while beginning transaction, error: '{0}'")]
    TransactionBeginError(sqlx::Error),
    #[error("error while commiting transaction, error: '{0}'")]
    TransactionCommitError(sqlx::Error),

    #[error("generic structure lib error: '{0}'")]
    GenericStructureError(#[from] starfoundry_lib_structures::Error),
    #[error("invalid typeId, '{0}'")]
    InvalidTypeId(TypeId),
}
