import { axiosClient, type AbortSignal } from "@internal/services/client";
import { useQuery } from "@tanstack/react-query";
import type { GenericAbortSignal } from "axios";
import type { Item } from "@internal/services/item/model";
import type { Structure } from "@internal/services/structure/list";
import type { Uuid } from "@internal/services/utils";

export const LIST_PROJECT_JOBS = 'listProjectJobs';

export const listProjectJobs = async (
    projectId: Uuid,
    signal?:   GenericAbortSignal,
): Promise<ProjectJobGroup[]> => (await axiosClient())
    .get(
        `/api/projects/${projectId}/jobs`,
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

export const useListProjectJobs = (
    projectId: Uuid,
) => {
    return useQuery({
        queryKey: [LIST_PROJECT_JOBS, projectId],
        queryFn: async ({
            signal
        }: AbortSignal) => listProjectJobs(projectId, signal),
        initialData: [],
    })
}

export type ProjectJobStatus = 'WAITING_FOR_MATERIALS' | 'BUILDING' | 'DONE';

export type ProjectJobGroup = {
    header:     string;
    entries:    ProjectJob[];
}

export type ProjectJob = {
    id:          Uuid;
    job_id?:     number;
    status:      ProjectJobStatus;
    runs:        number;
    cost?:       number;
    item:        Item;
    structure:   Structure;
    started_by?: number;
}
