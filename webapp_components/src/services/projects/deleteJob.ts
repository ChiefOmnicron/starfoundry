import { axiosClient } from "@internal/services/client";
import type { Uuid } from "@internal/services/utils";

export const deleteJob = async (
    projectId:  Uuid,
    jobId:      Uuid,
): Promise<void> => (await axiosClient())
    .delete(
        `/api/projects/${projectId}/jobs/${jobId}`,
    )
    .then(x => x.data);
