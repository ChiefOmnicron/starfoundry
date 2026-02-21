import { axiosClient } from "@internal/services/client";
import type { StructurePosition } from "./list";
import type { Uuid } from "@internal/services/utils";

export const createStructure = async (
    data: CreateStructure,
): Promise<CreateStructureResponse> => (await axiosClient())
    .post(
        '/api/structures',
        data,
    )
    .then(x => x.data);

export type CreateStructure = {
    name:               string;
    rigs:               number[];
    services:           number[];
    structure_id:       number;
    structure_type_id:  number;
    system_id:          number;
    position:           StructurePosition;
}

export type CreateStructureResponse = {
    id: Uuid,
}
