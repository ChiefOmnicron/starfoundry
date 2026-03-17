import { axiosClient } from "@internal/services/client";
import type { Uuid } from "@internal/services/utils";
import type { Item } from "@internal/services/item/model";

export const LIST_PROJECT = 'listProject';

export const checkResources = async (
    data: CheckMaterialsRequest,
): Promise<CheckMaterialsResponse> => (await axiosClient())
    .post(
        '/api/projects/check',
        data,
    )
    .then(x => {
        return x.data;
    });

export type CheckMaterialsRequest = {
    job_ids:        Uuid[];
    materials?:     Material[];
    materials_str?: string;
}

export type Material = {
    quantity: number;
    type_id:  number;
}

export type CheckMaterialsResponse = {
    job_cost:   number;
    materials:  CheckMaterialsResponseMaterial[];
    blueprints: CheckMaterialsResponseBlueprint[];
}

export type CheckMaterialsResponseMaterial = {
    item:     Item;
    quantity: number;
}

export type CheckMaterialsResponseBlueprint = {
    item:     Item;
    runs:     number[];
}
