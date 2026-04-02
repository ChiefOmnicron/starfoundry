import { axiosClient, type AbortSignal } from "@internal/services/client";
import { useQuery } from "@tanstack/react-query";
import type { GenericAbortSignal } from "axios";
import type { ProjectGroupMinimal } from "../project-group/list";
import type { Uuid } from "@internal/services/utils";

export const LIST_PROJECT = 'listProject';

export const listProjects = async (
    filter:  ProjectFilter,
    signal?: GenericAbortSignal,
): Promise<ProjectListMinimal[]> => (await axiosClient())
    .get(
        '/api/projects',
        {
            params: filter,
            signal,
        }
    )
    .then(x => {
        if (x.status === 204) {
            return [];
        }

        return x.data;
    });

export type ProjectFilter = {
    name?: string;
    status?: string;
    project_group_id?: string;
}

export const useListProjects = (
    filterParams: ProjectFilter,
) => {
    return useQuery({
        queryKey: [LIST_PROJECT, filterParams],
        queryFn: async ({
            signal
        }: AbortSignal) => listProjects(filterParams, signal),
        // 10 minutes (ms * s * m)
        staleTime: 1000 * 60 * 10,
    })
}

export type ProjectStatus = 'DRAFT' | 'READY_TO_START' | 'IN_PROGRESS' | 'PAUSED' | 'DONE';

export type ProjectListMinimal = {
    id:            Uuid;
    name:          string;
    status:        ProjectStatus;
    orderer:       string;
    sell_price:    number;
    project_group: ProjectGroupMinimal,
}
