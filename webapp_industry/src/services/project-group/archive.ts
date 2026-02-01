import { axiosClient } from "@/services/client";
import type { ProjectGroup } from "@/services/project-group/fetch";
import type { Uuid } from "@/services/utils";

export const archiveProjectGroup = async (
    projectGroupUuid: Uuid,
): Promise<ProjectGroup> => (await axiosClient())
    .put(
        `/api/project-groups/${projectGroupUuid}/archive`,
        {},
        {
            headers: {
                'Content-Type': 'application/json',
            }
        }
    )
    .then(x => x.data);
