import { axiosClient, type AbortSignal } from "../axiosClient";
import { useQuery } from "@tanstack/react-query";
import type { GenericAbortSignal } from "axios";
import type { ProjectJob } from "./listJobs";
import type { Uuid } from "../utils";

export const LIST_PROJECT_ALL_JOBS = 'listProjectAllJobs';

export const listProjectAllJobs = async (
    filter: ProjectAllJobFilter,
    signal?: GenericAbortSignal,
): Promise<ProjectJobAllGroup[]> => (await axiosClient())
    .get(
        `/api/projects/jobs`,
        {
            signal,
            params: filter,
        }
    )
    .then(x => {
        if (x.status === 204) {
            return [];
        }

        return x.data
    });

export const useListProjectAllJobs = (
    filters: ProjectAllJobFilter,
) => {
    return useQuery({
        queryKey: [LIST_PROJECT_ALL_JOBS, filters],
        queryFn: async ({
            signal
        }: AbortSignal) => listProjectAllJobs(filters, signal),
        // 10 minutes (ms * s * m)
        staleTime: 1000 * 60 * 10,
        // refetch it every 60 seconds
        refetchInterval: 60_000,
        refetchOnWindowFocus: false,
    })
}

export type ProjectJobAllGroup = {
    header:     string;
    project_id: Uuid,
    entries:    ProjectJob[];
}

export type ProjectAllJobFilter = {
    name?:  string;
    tags?:  string;
}
