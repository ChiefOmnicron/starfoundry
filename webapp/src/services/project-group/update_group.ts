import { axiosClient } from "@/services/client";
import type { ProjectGroup } from "./fetch";
import type { Uuid } from "@/services/utils";

export const CREATE_PROJECT_GROUPS = 'updateProjectGroup';

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
