import { axiosClient } from "@internal/services/client";
import type { TypeId, Uuid } from "@internal/services/utils";

export const addJobEntry = async (
    projectId:  Uuid,
    data:       AddJobEntryRequest[],
): Promise<void> => (await axiosClient())
    .post(
        `/api/projects/${projectId}/job`,
        data,
    )
    .then(x => x.data);

export type AddJobEntryRequest = {
    type_id:        TypeId;
    runs:           number;
    structure_id:   Uuid;
}
