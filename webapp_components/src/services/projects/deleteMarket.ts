import { axiosClient } from "../client";
import type { Uuid } from "../utils";

export const deleteMarketEntry = async (
    projectId:  Uuid,
    marketId:   Uuid,
): Promise<void> => (await axiosClient())
    .delete(
        `/api/projects/${projectId}/market/${marketId}`,
    )
    .then(x => x.data);
