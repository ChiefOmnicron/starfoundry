import { axiosClient } from "@internal/services/client";
import type { TypeId, Uuid } from "@internal/services/utils";

export const updateDefaultBlacklist = async (
    projectGroupUuid: Uuid,
    blacklist:        TypeId[],
): Promise<void> => (await axiosClient())
    .put(
        `/api/project-groups/${projectGroupUuid}/defaults/blacklist`,
        blacklist,
    );
