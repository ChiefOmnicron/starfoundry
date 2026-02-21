import { axiosClient, type AbortSignal } from "@internal/services/client";
import { useQuery } from "@tanstack/react-query";
import type { Item } from "../item/model";
import type { Uuid } from "@internal/services/utils";
import type { GenericAbortSignal } from "axios";

export const LIST_PROJECT_GROUP_DEFAULT_BLACKLIST = 'listProjectGroupsDefaultBlacklist';

export const listProjectGroupDefaultBlacklist = async (
    projectGroupUuid: Uuid,
    signal?:          GenericAbortSignal,
): Promise<Item[]> => (await axiosClient())
    .get(
        `/api/project-groups/${projectGroupUuid}/defaults/blacklist`,
        {
            signal,
        }
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
    queryFn: async ({
        signal
    }: AbortSignal) => listProjectGroupDefaultBlacklist(projectGroupUuid, signal),
});
