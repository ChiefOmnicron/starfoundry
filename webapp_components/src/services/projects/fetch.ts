import { axiosClient, type AbortSignal } from "@internal/services/client";
import { useQuery } from "@tanstack/react-query";
import type { Structure } from "@internal/services/structure/list";
import type { Uuid } from "@internal/services/utils";
import type { GenericAbortSignal } from "axios";

export const FETCH_PROJECT = 'fetchProject';

export const fetchProject = async (
    projectId: Uuid,
    signal?:   GenericAbortSignal,
): Promise<Structure> => (await axiosClient())
    .get(
        `/api/projects/${projectId}`,
        {
            signal,
        }
    )
    .then(x => x.data);

// For general use
export const useFetchProject = (
    id: Uuid,
) => {
    return useQuery(fetchProjectQuery(id));
}

// For pre-fetching
export const fetchProjectQuery = (
    id: Uuid,
) => ({
    queryKey: [FETCH_PROJECT, id],
    queryFn: async ({
        signal
    }: AbortSignal) => fetchProject(id, signal),
    // ms * s * m
    staleTime: 1000 * 60 * 5,
});
