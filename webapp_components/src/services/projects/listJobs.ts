import { axiosClient, type AbortSignal } from "@internal/services/client";
import { useQuery } from "@tanstack/react-query";
import type { GenericAbortSignal } from "axios";
import type { Item } from "@internal/services/item/model";
import type { Structure } from "@internal/services/structure/list";
import type { Uuid } from "@internal/services/utils";

export const LIST_PROJECT_JOBS = 'listProjectJobs';

export const listProjectJobs = async (
    projectId:  Uuid,
    filter:     ProjectJobFilter,
    signal?:    GenericAbortSignal,
): Promise<ProjectJobGroup[]> => (await axiosClient())
    .get(
        `/api/projects/${projectId}/jobs`,
        {
            signal,
            params: {
                ...filter,
            }
        }
    )
    .then(x => {
        if (x.status === 204) {
            return [];
        }

        return x.data
    });

export const useListProjectJobs = (
    projectId:  Uuid,
    filter:     ProjectJobFilter,
) => {
    return useQuery({
        queryKey: [LIST_PROJECT_JOBS, projectId],
        queryFn: async ({
            signal
        }: AbortSignal) => listProjectJobs(projectId, filter, signal),
        // 10 minutes (ms * s * m)
        staleTime: 1000 * 60 * 10,
    })
}

export const useListProjectJobsRefresh = (
    projectId:  Uuid,
    filter:     ProjectJobFilter,
) => {
    return useQuery({
        queryKey: [LIST_PROJECT_JOBS, projectId],
        queryFn: async ({
            signal
        }: AbortSignal) => listProjectJobs(projectId, filter, signal),
        staleTime: Infinity,
        // refetch it every 60 seconds
        refetchInterval: 60_000,
        refetchOnWindowFocus: false,
    });
}

export type ProjectJobStatus = 'WAITING_FOR_MATERIALS' | 'READY_TO_START' | 'BUILDING' | 'DONE';

export type ProjectJobGroup = {
    header:     string;
    entries:    ProjectJob[];
}

export type ProjectJob = {
    id:          Uuid;
    project_id:  Uuid;
    job_id?:     number;
    status:      ProjectJobStatus;
    runs:        number;
    cost?:       number;
    item:        Item;
    structure:   Structure;
    started_by?: number;
    end_date?:   string;
}

export type ProjectJobFilter = {
    startable?: boolean;
}
