import { axiosClient, type AbortSignal } from "@/services/client";
import { useQuery } from "@tanstack/react-query";
import type { Uuid } from "@/services/utils";
import type { GenericAbortSignal } from "axios";

export const LIST_PROJECT = 'listProject';

export const listProjects = async (
    filter:  ProjectFilter,
    signal?: GenericAbortSignal,
): Promise<ProjectList[]> => (await axiosClient())
    .get(
        '/api/projects',
        {
            params: filter,
            signal,
        }
    )
    .then(x => x.data);

export type ProjectFilter = {
    name?: string;
    status?: string;
    project_group?: string;
}

export const useListProjects = (
    filterParams: ProjectFilter,
) => {
    return useQuery({
        queryKey: [LIST_PROJECT, filterParams],
        queryFn: async ({
            signal
        }: AbortSignal) => listProjects(filterParams, signal),
        initialData: [],
    })
}

export type ProjectStatus = 'PREPARING' | 'IN_PROGRESS' | 'PAUSED' | 'DONE';

export type ProjectList = {
    id:         Uuid;
    name:       string;
    status:     ProjectStatus;
    orderer:    string;
    sell_price: number;
}
