import { axiosClient } from "../client";
import type { Uuid } from "../utils";

export const deleteProject = async (
    projectId: Uuid,
): Promise<void> => (await axiosClient())
    .delete(
        `/api/projects/${projectId}`,
    )
    .then(x => x.data);
