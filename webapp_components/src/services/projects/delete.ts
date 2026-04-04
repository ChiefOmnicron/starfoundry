import { axiosClient } from "@internal/services/client";
import type { Uuid } from "@internal/services/utils";

export const deleteProject = async (
    projectId: Uuid,
): Promise<void> => (await axiosClient())
    .delete(
        `/api/projects/${projectId}`,
    )
    .then(x => x.data);
