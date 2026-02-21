import { axiosClient, type AbortSignal } from "@internal/services/client";
import { useQuery } from "@tanstack/react-query";
import type { GenericAbortSignal } from "axios";
import type { Structure } from "@internal/services/structure/list";
import type { Uuid } from "@internal/services/utils";

export const LIST_INDUSTRY_HUB = 'listIndustryHub';

export const listIndustryHubs = async (
    filter:  IndustryHubFilter,
    signal?: GenericAbortSignal,
): Promise<IndustryHub[]> => (await axiosClient())
    .get(
        `/api/industry-hubs`,
        {
            params: filter,
            signal,
        }
    )
    .then(x => x.data);

export type IndustryHub = {
    id:           Uuid;
    name:         string;
    structures:   Structure[],
    shares:       IndustryHubShare[];
    description?: string,
}

export type IndustryHubShare = {
    name: string,
    share_id: number;
    share_type: 'CHARACTER' | 'CORPORATION' | 'ALLIANCE'
}

export type IndustryHubFilter = {
    name?: string;
    shared?: boolean;
}

// For general use
export const useListIndustryHub = (
    filter: IndustryHubFilter,
    enabled: boolean = true,
) => {
    return useQuery(listIndustryHubQuery(filter, enabled));
}

// For pre-loading
export const listIndustryHubQuery = (
    filter: IndustryHubFilter,
    enabled: boolean,
) => ({
    queryKey: [LIST_INDUSTRY_HUB, filter],
    queryFn: async ({
        signal,
    }: AbortSignal) => listIndustryHubs(filter, signal),
    enabled: enabled,
    // ms * s * m
    staleTime: 1000 * 60 * 5,
});
