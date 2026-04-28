import { axiosClient } from "@internal/services/client";
import type { Uuid } from "../utils"

export const updateJobAssignment = async (
    assignmentId: Uuid,
    jobId: Uuid,
): Promise<void> => (await axiosClient())
    .put(
        `/api/job-assignments/${assignmentId}/${jobId}`,
        {
            started: true,
        },
    )
    .then(x => x.data);
