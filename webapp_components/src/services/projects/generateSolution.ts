import { axiosClient } from "@internal/services/client";
import type { Item } from "@internal/services/item/model";
import type { Structure } from "@internal/services/structure/list";
import type { Uuid } from "../utils";
import type { IndustryHub } from "../industry-hub/list";

export const generateSolution = async (
    config: GenerateSolutionRequest,
): Promise<GenerateSolutionResponse[]> => (await axiosClient())
    .post(
        '/api/industry/calculation',
        config,
    )
    .then(x => x.data);

export type GenerateSolutionRequest = {
    project_group_id:           Uuid;
    products?:                  ProjectProducts[];
    products_str?:              string;
    additional_products?:       ProjectProducts[];
    additional_products_str?:   string;
    stocks_str?:                string;

    blacklist?:             number[];
    blueprint_overwrite?:   TmpBlueprintOverwrite[];
    job_splitting?:         TmpJobSplitting[];
}

export type TmpBlueprintOverwrite = {
    type_id:                number;
    material_efficiency:    number;
};

export type TmpJobSplitting = {
    type_id:    number;
    runs:       number;
};

export type ProjectProducts = {
    type_id:                number;
    quantity:               number;
    material_efficiency:    number;
}

export type GenerateSolutionResponse = {
    solution_id:    Uuid;
    industry_hub:   IndustryHub;
    material:       SolutionMaterial[];
    manufacturing:  SolutionManufacturing[];
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
