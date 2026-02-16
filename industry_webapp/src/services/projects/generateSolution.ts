import { axiosClient } from "@/services/client";
import type { Item } from "@/services/item/model";
import type { Structure } from "@/services/structure/list";
import type { Uuid } from "../utils";

export const generateSolution = async (
    projectGroupId: Uuid,
): Promise<GenerateSolutionResponse> => (await axiosClient())
    .post(
        '/api/industry/calculation',
        {
            project_group_id: projectGroupId,
            type_ids:         [23773],
        },
    )
    .then(x => x.data);

export type GenerateSolutionRequest = {
}

export type GenerateSolutionResponse = {
    material: SolutionMaterial[],
    manufacturing: SolutionManufacturing[];
}

export type SolutionMaterial = {
    item:   Item;
    needed: number;
    stock:  number;
}

export type SolutionManufacturing = {
    item:       Item,
    runs:       number[],
    build_tax:  number,
    structure?: Structure,
}
