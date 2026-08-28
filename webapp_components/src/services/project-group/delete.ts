import { axiosClient } from "../client";
import type { ProjectGroup } from "../project-group/fetch";
import type { Uuid } from "../utils";

export const deleteProjectGroup = async (
    projectGroupUuid: Uuid,
): Promise<ProjectGroup> => (await axiosClient())
    .delete(
        `/api/project-groups/${projectGroupUuid}`,
    )
    .then(x => x.data);
