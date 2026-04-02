import { axiosClient, type AbortSignal } from "@internal/services/client";
import { useQuery } from "@tanstack/react-query";
import type { GenericAbortSignal } from "axios";
import type { Uuid } from "@internal/services/utils";

export const LIST_PROJECT_GROUPS = 'listProjectGroups';

export const listProjectGroups = async (
    filter:  ProjectGroupFilter,
    signal?: GenericAbortSignal,
): Promise<ProjectGroupMinimal[]> => (await axiosClient())
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

export type ProjectGroupMinimal = {
    id:             Uuid,
    name:           string;
    project_count:  number;
    is_owner:       boolean;
    description?:   string;
    archived:       boolean,
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
