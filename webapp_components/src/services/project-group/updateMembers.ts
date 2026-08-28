import { axiosClient } from "../client";
import type { Uuid } from "../utils";

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
