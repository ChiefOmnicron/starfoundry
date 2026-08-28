import { axiosClient } from "../client";
import type { TypeId, Uuid } from "../utils";

export const addExcessEntry = async (
    projectId:  Uuid,
    data:       AddExcessEntryRequest[],
): Promise<void> => (await axiosClient())
    .post(
        `/api/projects/${projectId}/excess`,
        data,
    )
    .then(x => x.data);

export type AddExcessEntryRequest = {
    type_id:    TypeId;
    quantity:   number;
}
