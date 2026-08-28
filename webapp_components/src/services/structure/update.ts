import { axiosClient } from "../client";
import type { Uuid } from "../utils";
import type { StructureTax } from "./list";

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
    rigs:       number[];
    services:   number[];
    taxes:      StructureTax,
}

export type UpdateStructureResponse = {
    id: Uuid,
}
