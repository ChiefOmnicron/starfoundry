import axios from "axios";
import type { TypeId, Uuid } from "@/services/utils";
import { useQuery } from "@tanstack/react-query";

export const FETCH_PROJECT_GROUP_DEFAULT = 'fetchProjectGroupDefaults';

export const fetchProjectGroupDefaults = async (
    projectGroupUuid: Uuid,
): Promise<ProjectGroupDefault> => axios.get(
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
