import { axiosClient } from "@/services/client";
import { useQuery } from "@tanstack/react-query";
import type { Structure } from "../structure/list";
import type { Uuid } from "@/services/utils";

export const LIST_PROJECT_GROUP_DEFAULT_MARKETS = 'listProjectGroupDefaultMarkets';

export const listProjectGroupDefaultMarkets = async (
    projectGroupUuid: Uuid,
): Promise<Structure[]> => (await axiosClient())
    .get(
        `/api/project-groups/${projectGroupUuid}/defaults/markets`
    )
    .then(x => x.data);

// For general use
export const useListProjectGroupDefaultMarkets = (
    projectGroupUuid: Uuid,
) => {
    return useQuery(listProjectGroupDefaultMarketsQuery(projectGroupUuid));
}

// For pre-fetching
export const listProjectGroupDefaultMarketsQuery = (
    projectGroupUuid: Uuid,
) => ({
    queryKey: [LIST_PROJECT_GROUP_DEFAULT_MARKETS, projectGroupUuid],
    queryFn: async () => listProjectGroupDefaultMarkets(projectGroupUuid),
});
