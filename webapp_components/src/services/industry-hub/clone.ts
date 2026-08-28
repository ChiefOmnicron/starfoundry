import { axiosClient } from "../client";
import type { Uuid } from "../utils";

export const cloneIndustryHub = async (
    industryHubId: Uuid,
): Promise<Uuid> => (await axiosClient())
    .put(
        `/api/industry-hubs/${industryHubId}/clone`,
        {},
        {
            headers: {
                'Content-Type': 'application/json',
            }
        }
    )
    .then(x => x.data.id);
