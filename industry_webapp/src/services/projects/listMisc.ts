import { axiosClient, type AbortSignal } from "@/services/client";
import { useQuery } from "@tanstack/react-query";
import type { GenericAbortSignal } from "axios";
import type { Uuid } from "@/services/utils";

export const LIST_PROJECT_JOBS = 'listProjectMisc';

export const listProjectMisc = async (
    projectId: Uuid,
    signal?:   GenericAbortSignal,
): Promise<ProjectMisc[]> => (await axiosClient())
    .get(
        `/api/projects/${projectId}/misc`,
        {
            signal,
        }
    )
    .then(x => x.data);

export const useListProjectMisc = (
    projectId: Uuid,
) => {
    return useQuery({
        queryKey: [LIST_PROJECT_JOBS, projectId],
        queryFn: async ({
            signal
        }: AbortSignal) => listProjectMisc(projectId, signal),
        initialData: [],
    })
}

export type ProjectMisc = {
    item:         string;
    cost:         number;

    description?: string;
    quantity?:    number;
}
