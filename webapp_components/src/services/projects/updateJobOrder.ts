import { axiosClient } from "@internal/services/client";
import type { Uuid } from "../utils"

export const updateJobOrder = async (
    assignmentId: Uuid,
    jobId: Uuid,
): Promise<void> => (await axiosClient())
    .put(
        `/api/projects/job-assignments/${assignmentId}/${jobId}`,
        {
            started: true,
        },
    )
    .then(x => x.data);
