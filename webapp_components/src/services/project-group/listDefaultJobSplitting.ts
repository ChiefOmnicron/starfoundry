import { axiosClient, type AbortSignal } from "@internal/services/client";
import { useQuery } from "@tanstack/react-query";
import type { Item } from "../item/model";
import type { Uuid } from "@internal/services/utils";
import type { GenericAbortSignal } from "axios";

export const LIST_PROJECT_GROUP_DEFAULT_JOB_SPLITTING = 'listProjectGroupsDefaultJobSplitting';

export const listProjectGroupDefaultJobSplitting = async (
    projectGroupUuid: Uuid,
    signal?:          GenericAbortSignal,
): Promise<JobSplitting> => (await axiosClient())
    .get(
        `/api/project-groups/${projectGroupUuid}/defaults/job-splitting`,
        {
            signal,
        }
    )
    .then(x => {
        if (x.status === 204) {
            return [];
        } else {
            return x.data;
        }
    });

export type JobSplitting = {
    general: JobSplittingGeneral,
    runs:    JobSplittingRun[],
}

export type JobSplittingGeneral = {
    manufacturing: number,
    reaction:      number,
}

export type JobSplittingRun = {
    max_runs: number,
    item:     Item,
}

// For general use
export const useListProjectGroupDefaultJobSplitting = (
    projectGroupUuid:   Uuid,
    options:            AdditionalProjectGroupDefaultJobSplittingOptions = {},
) => {
    return useQuery(listProjectGroupDefaultJobSplittingQuery(projectGroupUuid, options));
}

// For pre-fetching
export const listProjectGroupDefaultJobSplittingQuery = (
    projectGroupUuid:   Uuid,
    options:            AdditionalProjectGroupDefaultJobSplittingOptions,
) => ({
    queryKey: [LIST_PROJECT_GROUP_DEFAULT_JOB_SPLITTING, projectGroupUuid],
    queryFn: async ({
        signal
    }: AbortSignal) => listProjectGroupDefaultJobSplitting(projectGroupUuid, signal),
    ...options,
});

export type AdditionalProjectGroupDefaultJobSplittingOptions = {
    enabled?: boolean,
}
