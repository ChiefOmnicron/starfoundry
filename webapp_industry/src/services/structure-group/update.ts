import { axiosClient } from "@/services/client";
import type { Uuid } from "@/services/utils";

export const updateStructureGroup = async (
    structureGroupId: Uuid,
    data:             UpdateStructureGroup,
): Promise<Uuid> => (await axiosClient())
    .put(
        `/api/structure-groups/${structureGroupId}`,
        data,
    )
    .then(x => x.data.id);

export type UpdateStructureGroup = {
    name:       string,
    structures: Uuid[],
}
