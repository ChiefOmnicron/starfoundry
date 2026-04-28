import { axiosClient } from "@internal/services/client";
import type { Uuid } from "../utils"

export const createJobAssignment = async (
    data: ProjectJobMinimal[],
): Promise<ProjectJobOrderResponse> => (await axiosClient())
    .post(
        '/api/job-assignments',
        data,
    )
    .then(x => x.data);

export type ProjectJobMinimal = {
    project_id: Uuid,
    job_id:     Uuid,
}

export type ProjectJobOrderResponse = {
    id: Uuid;
}
