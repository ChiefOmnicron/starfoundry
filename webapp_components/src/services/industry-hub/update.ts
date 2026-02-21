import { axiosClient } from "@internal/services/client";
import type { IndustryHubShare } from "@internal/services/industry-hub/list";
import type { Uuid } from "@internal/services/utils";

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
