import { axiosClient } from "@internal/services/client";
import type { ProjectGroup } from "@internal/services/project-group/fetch";
import type { Uuid } from "@internal/services/utils";

export const deleteProjectGroup = async (
    projectGroupUuid: Uuid,
): Promise<ProjectGroup> => (await axiosClient())
    .delete(
        `/api/project-groups/${projectGroupUuid}`,
    )
    .then(x => x.data);
