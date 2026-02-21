import { axiosClient } from "@internal/services/client";
import type { Uuid } from "@internal/services/utils";

export const createIndustryHub = async (
    data: CreateIndustryHub,
): Promise<CreateIndustryHubResponse> => (await axiosClient())
    .post(
        '/api/industry-hubs',
        data,
    )
    .then(x => x.data);

export type CreateIndustryHub = {
    name: string;
}

export type CreateIndustryHubResponse = {
    id: Uuid,
}
