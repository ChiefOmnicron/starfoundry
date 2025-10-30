import { axiosClient } from "@/services/client";
import type { Uuid } from "@/services/utils";

export const updateStructure = async (
    structureId: Uuid,
    data:        UpdateStructure,
): Promise<UpdateStructureResponse> => (await axiosClient())
    .put(
        `/api/structures/${structureId}`,
        data,
    )
    .then(x => x.data);

export type UpdateStructure = {
    rigs:               number[];
    services:           number[];
}

export type UpdateStructureResponse = {
    id: Uuid,
}
