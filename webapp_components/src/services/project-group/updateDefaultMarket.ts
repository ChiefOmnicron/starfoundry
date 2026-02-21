import { axiosClient } from "@internal/services/client";
import type { Uuid } from "@internal/services/utils";

export const updateDefaultMarket = async (
    projectGroupUuid: Uuid,
    markets:          Uuid[],
): Promise<void> => (await axiosClient())
    .put(
        `/api/project-groups/${projectGroupUuid}/defaults/market`,
        markets,
    );
