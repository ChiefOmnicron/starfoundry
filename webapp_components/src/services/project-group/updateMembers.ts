import { axiosClient } from "@internal/services/client";
import type { Uuid } from "@internal/services/utils";

export const updateMembers = async (
    projectGroupUuid: Uuid,
    characterIds:     UpdateMemberRequest[],
): Promise<void> => (await axiosClient())
    .put(
        `/api/project-groups/${projectGroupUuid}/members`,
        characterIds,
    );

export type UpdateMemberRequest = {
    character_id: number;
}
