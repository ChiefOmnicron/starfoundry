import { axiosClient } from "@internal/services/client";
import type { Uuid } from "@internal/services/utils";

export const deleteStructure = async (
    structureId: Uuid,
): Promise<Uuid> => (await axiosClient())
    .delete(
        `/api/structures/${structureId}`,
    )
    .then(x => x.data.id);
