import { axiosClient, type AbortSignal } from "@/services/client";
import { useQuery } from "@tanstack/react-query";
import type { GenericAbortSignal } from "axios";
import type { IndustryHub } from "@/services/industry-hub/list";
import type { Uuid } from "@/services/utils";

export const LIST_PROJECT_GROUP_INDUSTRY_HUBS = 'listProjectGroupIndustryHubs';

export const listProjectGroupIndustryHubs = async (
    projectGroupUuid: Uuid,
    signal?:          GenericAbortSignal,
): Promise<IndustryHub[]> => (await axiosClient())
    .get(
        `/api/project-groups/${projectGroupUuid}/industry-hubs`,
        {
            signal,
        }
    )
    .then(x => x.data);

// For general use
export const useListProjectGroupIndustryHubs = (
    projectGroupUuid: Uuid,
) => {
    return useQuery(listProjectGroupIndustryHubsQuery(projectGroupUuid));
}

// For pre-fetching
export const listProjectGroupIndustryHubsQuery = (
    projectGroupUuid: Uuid,
) => ({
    queryKey: [LIST_PROJECT_GROUP_INDUSTRY_HUBS, projectGroupUuid],
    queryFn: async ({
        signal
    }: AbortSignal) => listProjectGroupIndustryHubs(projectGroupUuid, signal),
});
