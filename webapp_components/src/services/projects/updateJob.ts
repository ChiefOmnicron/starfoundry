import { axiosClient } from "../client";
import type { ProjectJobStatus } from "../projects/listJobs";
import type { Uuid } from "../utils";

export const updateProjectJob = async (
    projectId:      Uuid,
    projectJobId:   Uuid,
    data:           UpdateProjectJob,
): Promise<void> => (await axiosClient())
    .put(
        `/api/projects/${projectId}/jobs/${projectJobId}`,
        data,
    )
    .then(x => {
        return x.data;
    });

export type UpdateProjectJob = {
    cost?:      number;
    status:     ProjectJobStatus;
    job_id?:    number;
    runs?:      number;
}
