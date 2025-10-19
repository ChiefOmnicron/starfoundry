import { axiosClient } from "@/services/client";
import type { ProjectGroup } from "./fetch";
import type { Uuid } from "@/services/utils";

export const CREATE_PROJECT_GROUPS = 'updateProjectGroup';

export const deleteProjectGroup = async (
    projectGroupUuid: Uuid,
): Promise<ProjectGroup> => (await axiosClient())
    .delete(
        `/api/project-groups/${projectGroupUuid}`,
    )
    .then(x => x.data);
