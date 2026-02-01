import { axiosClient, type AbortSignal } from "@/services/client";
import { useQuery } from "@tanstack/react-query";
import type { ProjectGroup } from "@/services/project-group/fetch";
import type { GenericAbortSignal } from "axios";

export const LIST_PROJECT_GROUPS = 'listProjectGroups';

export const listProjectGroups = async (
    filter:  ProjectGroupFilter,
    signal?: GenericAbortSignal,
): Promise<ProjectGroup[]> => (await axiosClient())
    .get(
        '/api/project-groups',
        {
            params: filter,
            signal,
        }
    )
    .then(x => x.data);

export type ProjectGroupFilter = {
    name?: string;
    owner?: boolean;
    archived?: boolean;
}

export const useListProjectGroup = (
    filterParams: ProjectGroupFilter,
) => {
    return useQuery({
        queryKey: [LIST_PROJECT_GROUPS, filterParams],
        queryFn: async ({
            signal
        }: AbortSignal) => listProjectGroups(filterParams, signal),
        initialData: [],
    })
}
