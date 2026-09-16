import { axiosClient } from "../axiosClient";
import type { Uuid } from "../utils";

export const deleteJob = async (
    projectId:  Uuid,
    jobId:      Uuid,
): Promise<void> => (await axiosClient())
    .delete(
        `/api/projects/${projectId}/jobs/${jobId}`,
    )
    .then(x => x.data);
