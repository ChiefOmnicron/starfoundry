import { axiosClient, type AbortSignal } from "@internal/services/client";
import { useQuery } from "@tanstack/react-query";
import type { GenericAbortSignal } from "axios";
import type { ProjectJob } from "./listJobs";
import type { Uuid } from "../utils";

export const LIST_PROJECT_ALL_JOBS = 'listProjectAllJobs';

export const listProjectAllJobs = async (
    signal?: GenericAbortSignal,
): Promise<ProjectJobAllGroup[]> => (await axiosClient())
    .get(
        `/api/projects/jobs`,
        {
            signal,
        }
    )
    .then(x => {
        if (x.status === 204) {
            return [];
        }

        return x.data
    });

export const useListProjectAllJobs = () => {
    return useQuery({
        queryKey: [LIST_PROJECT_ALL_JOBS],
        queryFn: async ({
            signal
        }: AbortSignal) => listProjectAllJobs(signal),
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
