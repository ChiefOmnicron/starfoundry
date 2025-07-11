import { axiosClient } from "@/services/client";
import { useQuery } from "@tanstack/react-query";
import type { TypeId, Uuid } from "@/services/utils";

export const FETCH_PROJECT_GROUP_DEFAULT = 'fetchProjectGroupDefaults';

export const fetchProjectGroupDefaults = async (
    projectGroupUuid: Uuid,
): Promise<ProjectGroupDefault> => (await axiosClient())
    .get(
        `/api/project-groups/${projectGroupUuid}/defaults`
    )
    .then(x => x.data);

export type ProjectGroupDefault = {
    markets: Uuid[];

    blacklist: TypeId[];
}

// For general use
export const useFetchProjectGroupDefaults = (
    projectGroupUuid: Uuid,
) => {
    return useQuery(fetchProjectGroupDefaultsQuery(projectGroupUuid));
}

// For pre-fetching
export const fetchProjectGroupDefaultsQuery = (
    projectGroupUuid: Uuid,
) => ({
    queryKey: [FETCH_PROJECT_GROUP_DEFAULT, projectGroupUuid],
    queryFn: async () => fetchProjectGroupDefaults(projectGroupUuid),
});
