import { axiosClient } from "@/services/client";
import type { Uuid } from "@/services/utils";

export const createStructureGroup = async (
    data: CreateStructureGroup,
): Promise<CreateStructureGroupResponse> => (await axiosClient())
    .post(
        '/api/structure-groups',
        data,
    )
    .then(x => x.data);

export type CreateStructureGroup = {
    name: string;
}

export type CreateStructureGroupResponse = {
    id: Uuid,
}
