import { axiosClient } from "@internal/services/client";
import type { ProjectGroup } from "@internal/services/project-group/fetch";
import type { Uuid } from "@internal/services/utils";

export const updateProjectGroup = async (
    projectGroupUuid: Uuid,
    data: UpdateProjectGroup,
): Promise<ProjectGroup> => (await axiosClient())
    .put(
        `/api/project-groups/${projectGroupUuid}`,
        data,
    )
    .then(x => x.data);

export interface UpdateProjectGroup {
    name: string;
    description?: string;
}
