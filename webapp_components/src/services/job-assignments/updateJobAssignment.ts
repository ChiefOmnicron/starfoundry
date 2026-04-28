import axios from "axios";
import type { Uuid } from "../utils"

export const updateJobAssignment = async (
    assignmentId: Uuid,
    jobId: Uuid,
): Promise<void> => axios
    .put(
        `/api/job-assignments/${assignmentId}/${jobId}`,
        {
            started: true,
        },
    )
    .then(x => x.data);
