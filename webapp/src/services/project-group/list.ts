import { axiosClient } from "@/services/client";
import { useQuery } from "@tanstack/react-query";
import type { ProjectGroup } from "./fetch";

export const LIST_PROJECT_GROUPS = 'listProjectGroups';

export const listProjectGroups = async (
    filter: ProjectGroupFilter,
): Promise<ProjectGroup[]> => (await axiosClient())
    .get(
        '/api/project-groups',
        {
            params: filter,
        }
    )
    .then(x => x.data);

export type ProjectGroupFilter = {
    name?: string;
    owner?: boolean;
}

export const useListProjectGroup = (
    filterParams: ProjectGroupFilter,
) => {
    return useQuery({
        queryKey: [LIST_PROJECT_GROUPS, filterParams],
        queryFn: async () => listProjectGroups(filterParams),
        initialData: [],
    })
}
