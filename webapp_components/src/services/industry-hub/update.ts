import { axiosClient } from "../client";
import type { IndustryHubShare } from "../industry-hub/list";
import type { Uuid } from "../utils";

export const updateIndustryHub = async (
    industryHubId: Uuid,
    data:          UpdateIndustryHub,
): Promise<Uuid> => (await axiosClient())
    .put(
        `/api/industry-hubs/${industryHubId}`,
        data,
    )
    .then(x => x.data.id);

export type UpdateIndustryHub = {
    name:         string,
    structures:   Uuid[],
    shares:       IndustryHubShare[];
    description?: string;
}
