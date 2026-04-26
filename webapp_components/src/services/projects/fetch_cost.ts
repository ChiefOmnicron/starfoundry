import { axiosClient, type AbortSignal } from "@internal/services/client";
import { useQuery } from "@tanstack/react-query";
import type { GenericAbortSignal } from "axios";
import type { Uuid } from "@internal/services/utils";

export const FETCH_PROJECT_COST = 'fetchProjectCost';

export const fetchProjectCost = async (
    projectId: Uuid,
    signal?:   GenericAbortSignal,
): Promise<ProjectCost> => (await axiosClient())
    .get(
        `/api/projects/${projectId}/cost`,
        {
            signal,
        }
    )
    .then(x => x.data);

// For general use
export const useFetchProjectCost = (
    id: Uuid,
) => {
    return useQuery(fetchProjectCostQuery(id));
}

// For pre-fetching
export const fetchProjectCostQuery = (
    id: Uuid,
) => ({
    queryKey: [FETCH_PROJECT_COST, id],
    queryFn: async ({
        signal
    }: AbortSignal) => fetchProjectCost(id, signal),
    // ms * s * m
    staleTime: 1000 * 60 * 5,
});

export type ProjectCost = {
    sell_price:     number;

    job_cost:       number;
    market_cost:    number;
    misc_cost:      number;
    excess_cost:    number;
    stock_cost:     number;
}
