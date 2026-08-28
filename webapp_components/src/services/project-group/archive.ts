import { axiosClient } from "../client";
import type { ProjectGroup } from "../project-group/fetch";
import type { Uuid } from "../utils";

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
