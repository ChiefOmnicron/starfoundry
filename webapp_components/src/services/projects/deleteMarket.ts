import { axiosClient } from "@internal/services/client";
import type { Uuid } from "@internal/services/utils";

export const deleteMarketEntry = async (
    projectId:  Uuid,
    marketId:   Uuid,
): Promise<void> => (await axiosClient())
    .delete(
        `/api/projects/${projectId}/market/${marketId}`,
    )
    .then(x => x.data);
