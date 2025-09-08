import { axiosClient } from "@/services/client";
import { useQuery } from "@tanstack/react-query";
import type { Item } from "../item/model";
import type { ProjectGroupMember } from "./list_members";
import type { Structure } from "../structure/list";
import type { Uuid } from "@/services/utils";

export const FETCH_PROJECT_GROUP = 'fetchProjectGroup';

export const fetchProjectGroup = async (
    projectGroupUuid: Uuid,
): Promise<ProjectGroup> => (await axiosClient())
    .get(
        `/api/project-groups/${projectGroupUuid}`
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
    queryFn: async () => fetchProjectGroup(projectGroupUuid),
});
