import { axiosClient } from "@/services/client";
import type { Uuid } from "@/services/utils";

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
    name:       string,
    structures: Uuid[],
}
