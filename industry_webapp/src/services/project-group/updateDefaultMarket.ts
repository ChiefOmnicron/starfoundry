import { axiosClient } from "@/services/client";
import type { Uuid } from "@/services/utils";

export const updateDefaultMarket = async (
    projectGroupUuid: Uuid,
    markets:          Uuid[],
): Promise<void> => (await axiosClient())
    .put(
        `/api/project-groups/${projectGroupUuid}/defaults/market`,
        markets,
    );
