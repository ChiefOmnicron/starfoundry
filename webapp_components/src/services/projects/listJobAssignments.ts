import { axiosClient, type AbortSignal } from "@internal/services/client";
import { useQuery } from "@tanstack/react-query";
import type { GenericAbortSignal } from "axios";
import type { Item } from "@internal/services/item/model";
import type { Uuid } from "@internal/services/utils";

export const LIST_JOB_ASSIGNMENT = 'listJobAssignment';

export const listJobAssignments = async (
    assignment_id:  Uuid,
    signal?:        GenericAbortSignal,
): Promise<ProjectJobAssignmentGroup[]> => (await axiosClient())
    .get(
        `/api/projects/job-assignments/${assignment_id}`,
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

export const useListJobAssignmentsRefresh = (
    assignment_id: Uuid,
) => {
    return useQuery({
        queryKey: [LIST_JOB_ASSIGNMENT, assignment_id],
        queryFn: async ({
            signal
        }: AbortSignal) => listJobAssignments(assignment_id, signal),
        staleTime: Infinity,
        // refetch it every 5 seconds
        refetchInterval: 5_000,
        refetchOnWindowFocus: false,
    });
}

export type ProjectJobAssignmentGroup = {
    header:     string;
    entries:    ProjectJobAssignment[];
}

export type ProjectJobAssignment = {
    id:             Uuid,
    structure_name: string,
    started:        boolean,
    item:           Item,
    runs:           number,
}
