import { axiosClient } from "../client";
import type { TypeId, Uuid } from "../utils";

export const updateDefaultBlacklist = async (
    projectGroupUuid: Uuid,
    blacklist:        TypeId[],
): Promise<void> => (await axiosClient())
    .put(
        `/api/project-groups/${projectGroupUuid}/defaults/blacklist`,
        blacklist,
    );
