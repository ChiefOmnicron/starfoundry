import { axiosClient } from "../client";
import type { Uuid } from "../utils";

export const InitializeProject = async (
    projectId:  Uuid,
    config:     InitializeProjectRequest,
): Promise<void> => (await axiosClient())
    .put(
        `/api/projects/${projectId}/initialize`,
        config,
    )
    .then(x => x.data);

export type InitializeProjectRequest = {
    solution_id: Uuid;
}
