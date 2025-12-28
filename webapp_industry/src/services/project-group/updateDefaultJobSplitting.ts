import { axiosClient } from "@/services/client";
import type { TypeId, Uuid } from "@/services/utils";

export const updateDefaultJobSplitting = async (
    projectGroupUuid: Uuid,
    jobSplitting:     UpdateJobSplittingRun[],
): Promise<void> => (await axiosClient())
    .put(
        `/api/project-groups/${projectGroupUuid}/defaults/job-splitting`,
        jobSplitting,
    );

export type UpdateJobSplittingRun = {
    max_runs: number,
    type_id:  TypeId,
}
