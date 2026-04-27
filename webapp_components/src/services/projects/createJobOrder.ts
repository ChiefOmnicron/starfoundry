import { axiosClient } from "@internal/services/client";
import type { Uuid } from "../utils"

export const createJobOrder = async (
    data: ProjectJobMinimal[],
): Promise<ProjectJobOrderResponse> => (await axiosClient())
    .post(
        '/api/projects/job-assignments',
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
