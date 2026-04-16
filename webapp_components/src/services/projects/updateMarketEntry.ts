import { axiosClient } from "@internal/services/client";
import type { Uuid } from "@internal/services/utils";

export const updateMarketEntry = async (
    projectId:  Uuid,
    marketId:   Uuid,
    data:       UpdateMarketEntry,
): Promise<void> => (await axiosClient())
    .put(
        `/api/projects/${projectId}/market/${marketId}`,
        data,
    )
    .then(x => {
        return x.data;
    });

export type UpdateMarketEntry = {
    quantity:   number;
    source?:    string;
    cost?:      number;
}
