import { axiosClient } from "@internal/services/client";
import type { TypeId, Uuid } from "@internal/services/utils";

export const addMarketEntry = async (
    projectId:  Uuid,
    data:       AddMarketEntryRequest[],
): Promise<void> => (await axiosClient())
    .post(
        `/api/projects/${projectId}/market`,
        data,
    )
    .then(x => x.data);

export type AddMarketEntryRequest = {
    type_id:    TypeId;
    quantity:   number;
}
