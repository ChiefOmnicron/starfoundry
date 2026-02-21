import { axiosClient, type AbortSignal } from "@internal/services/client";
import { useQuery } from "@tanstack/react-query";
import type { Uuid } from "@internal/services/utils";
import type { GenericAbortSignal } from "axios";
import type { ProjectGroup } from "../project-group/fetch";

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
        initialData: [],
    })
}

export type ProjectStatus = 'DRAFT' | 'READY_TO_START' | 'IN_PROGRESS' | 'PAUSED' | 'DONE';

export type ProjectList = {
    id:            Uuid;
    name:          string;
    status:        ProjectStatus;
    orderer:       string;
    sell_price:    number;
    project_group: ProjectGroup,
}
