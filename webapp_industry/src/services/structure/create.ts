import { axiosClient } from "@/services/client";
import type { Uuid } from "@/services/utils";

export const CREATE_STRUCTURE = 'createStructure';

export const createStructure = async (
    data: CreateStructure,
): Promise<CreateStructureResponse> => (await axiosClient())
    .post(
        '/api/industry/structures',
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
}

export type CreateStructureResponse = {
    id: Uuid,
}
