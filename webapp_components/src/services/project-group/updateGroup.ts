import { axiosClient } from "../client";
import type { ProjectGroup } from "../project-group/fetch";
import type { Uuid } from "../utils";

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
