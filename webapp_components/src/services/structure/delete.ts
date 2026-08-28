import { axiosClient } from "../client";
import type { Uuid } from "../utils";

export const deleteStructure = async (
    structureId: Uuid,
): Promise<Uuid> => (await axiosClient())
    .delete(
        `/api/structures/${structureId}`,
    )
    .then(x => x.data.id);
