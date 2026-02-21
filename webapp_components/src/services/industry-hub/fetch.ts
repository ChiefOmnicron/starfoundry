import { axiosClient, type AbortSignal } from "@internal/services/client";
import { useQuery } from "@tanstack/react-query";
import type { GenericAbortSignal } from "axios";
import type { IndustryHub } from "@internal/services/industry-hub/list";
import type { Uuid } from "@internal/services/utils";

export const FETCH_INDUSTRY_HUB = 'fetchIndustryHub';

export const fetchIndustryHub = async (
    industryHubId: Uuid,
    signal?:       GenericAbortSignal,
): Promise<IndustryHub> => (await axiosClient())
    .get(
        `/api/industry-hubs/${industryHubId}`,
        {
            signal
        },
    )
    .then(x => x.data);

// For general use
export const useFetchIndustryHub = (
    id: Uuid,
) => {
    return useQuery(fetchIndustryHubQuery(id));
}

// For pre-fetching
export const fetchIndustryHubQuery = (
    id: Uuid,
) => ({
    queryKey: [FETCH_INDUSTRY_HUB, id],
    queryFn: async ({
        signal
    }: AbortSignal) => fetchIndustryHub(id, signal),
    // ms * s * m
    staleTime: 1000 * 60 * 5,
});
