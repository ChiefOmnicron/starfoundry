import { axiosClient } from "@/services/client";
import { useQuery } from "@tanstack/react-query";
import type { ProjectGroup } from "./fetch";
import type { Uuid } from "@/services/utils";

export const LIST_PROJECT_GROUP_DEFAULT_BLACKLIST = 'listProjectGroupsDefaultBlacklist';

export const listProjectGroupDefaultBlacklist = async (
    projectGroupUuid: Uuid,
): Promise<ProjectGroup[]> => (await axiosClient())
    .get(
        `/api/project-groups/${projectGroupUuid}/defaults/blacklist`
    )
    .then(x => x.data);

// For general use
export const useListProjectGroupDefaultBlacklist = (
    projectGroupUuid: Uuid,
) => {
    return useQuery(listProjectGroupDefaultBlacklistQuery(projectGroupUuid));
}

// For pre-fetching
export const listProjectGroupDefaultBlacklistQuery = (
    projectGroupUuid: Uuid,
) => ({
    queryKey: [LIST_PROJECT_GROUP_DEFAULT_BLACKLIST, projectGroupUuid],
    queryFn: async () => listProjectGroupDefaultBlacklist(projectGroupUuid),
});
