import { axiosClient } from "@/services/client";
import type { Uuid } from "@/services/utils";

export const deleteStructure = async (
    structureId: Uuid,
): Promise<Uuid> => (await axiosClient())
    .delete(
        `/api/structures/${structureId}`,
    )
    .then(x => x.data.id);
