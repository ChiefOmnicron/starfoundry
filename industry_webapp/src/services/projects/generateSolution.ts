import { axiosClient } from "@/services/client";
import type { Item } from "@/services/item/model";
import type { Structure } from "@/services/structure/list";

export const generateSolution = async (
): Promise<GenerateSolutionResponse> => (await axiosClient())
    .post(
        '/api/industry/calculation',
        {
            project_group_id: '0196bc79-d070-7a4c-86d0-686ed6d5cc32',
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
