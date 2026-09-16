import { axiosClient } from "../axiosClient";
import type { Uuid } from "../utils";

export const updateDefaultMarket = async (
    projectGroupUuid: Uuid,
    markets:          Uuid[],
): Promise<void> => (await axiosClient())
    .put(
        `/api/project-groups/${projectGroupUuid}/defaults/market`,
        markets,
    );
