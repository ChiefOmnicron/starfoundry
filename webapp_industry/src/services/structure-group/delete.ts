import { axiosClient } from "@/services/client";
import type { Uuid } from "@/services/utils";

export const deleteStructureGroup = async (
    structureGroupId: Uuid,
): Promise<Uuid> => (await axiosClient())
    .delete(
        `/api/structure-groups/${structureGroupId}`,
    )
    .then(x => x.data.id);
