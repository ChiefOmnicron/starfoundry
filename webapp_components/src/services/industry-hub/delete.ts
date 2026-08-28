import { axiosClient } from "../client";
import type { Uuid } from "../utils";

export const deleteIndustryHub = async (
    industryHubId: Uuid,
): Promise<Uuid> => (await axiosClient())
    .delete(
        `/api/industry-hubs/${industryHubId}`,
    )
    .then(x => x.data.id);
