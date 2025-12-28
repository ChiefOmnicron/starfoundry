import { axiosClient, type AbortSignal } from "@/services/client";
import { useQuery } from "@tanstack/react-query";
import type { Item } from "@/services/item/model";
import type { ProjectGroupMember } from "@/services/project-group/listMembers";
import type { Structure } from "@/services/structure/list";
import type { Uuid } from "@/services/utils";
import type { GenericAbortSignal } from "axios";

export const FETCH_PROJECT_GROUP = 'fetchProjectGroup';

export const fetchProjectGroup = async (
    projectGroupUuid: Uuid,
    signal?:          GenericAbortSignal,
): Promise<ProjectGroup> => (await axiosClient())
    .get(
        `/api/project-groups/${projectGroupUuid}`,
        {
            signal,
        }
    )
    .then(x => x.data);

export type ProjectGroup = {
    id: Uuid,
    name: string;
    project_count: number;
    is_owner: boolean;
    description?: string;

    members: ProjectGroupMember[];
    default_market: Structure[];
    default_blacklist: Item[];
}

// For general use
export const useFetchProjectGroup = (
    projectGroupUuid: Uuid,
) => {
    return useQuery(fetchProjectGroupQuery(projectGroupUuid));
}

// For pre-fetching
export const fetchProjectGroupQuery = (
    projectGroupUuid: Uuid,
) => ({
    queryKey: [FETCH_PROJECT_GROUP, projectGroupUuid],
    queryFn: async ({
        signal
    }: AbortSignal) => fetchProjectGroup(projectGroupUuid, signal),
    // ms * s * m
    staleTime: 1000 * 60 * 5,
});
