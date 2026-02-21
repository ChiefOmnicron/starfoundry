import { axiosClient } from "@internal/services/client";
import type { ProjectGroup } from "@internal/services/project-group/fetch";
import type { Uuid } from "@internal/services/utils";

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
