import { axiosClient } from "../client";
import type { Item } from "../item/model";
import type { Structure } from "../structure/list";
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
    calculate_market_cost?:     boolean;

    blacklist?:             number[];
    blueprint_overwrite?:   TmpBlueprintOverwrite[];
    job_splitting?:         TmpJobSplitting[];
    markets?:               number[];
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
    price?: number;
}

export type SolutionManufacturing = {
    item:       Item,
    runs:       number[],
    build_tax:  number,
    structure?: Structure,
}
