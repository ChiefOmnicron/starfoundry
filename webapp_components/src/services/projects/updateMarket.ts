import { axiosClient } from "@internal/services/client";
import type { Uuid } from "@internal/services/utils";

export const updateMarketBulk = async (
    projectId:  Uuid,
    data:       UpdateMarketRequest,
): Promise<void> => (await axiosClient())
    .put(
        `/api/projects/${projectId}/market`,
        data,
    )
    .then(x => {
        return x.data;
    });

export type UpdateMarketRequest = {
    source:                 string;
    entries:                UpdateMarketEntry[];

    gas_decompression?:     string;
    mineral_compression?:   string;
}

export type UpdateMarketEntry = {
    type_id?:       number;
    name?:          string;
    structure_id?:  number;

    cost:           number;
    quantity:       number;
}
