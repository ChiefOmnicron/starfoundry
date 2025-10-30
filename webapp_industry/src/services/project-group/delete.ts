import { axiosClient } from "@/services/client";
import type { ProjectGroup } from "@/services/project-group/fetch";
import type { Uuid } from "@/services/utils";

export const deleteProjectGroup = async (
    projectGroupUuid: Uuid,
): Promise<ProjectGroup> => (await axiosClient())
    .delete(
        `/api/project-groups/${projectGroupUuid}`,
    )
    .then(x => x.data);
