import { axiosClient, type AbortSignal } from "@internal/services/client";
import { useQuery } from "@tanstack/react-query";
import type { GenericAbortSignal } from "axios";
import type { Uuid } from "@internal/services/utils";
import type { Item } from "@internal/services/item/model";

export const LIST_PROJECT_MARKET = 'listProjectMarket';

export const listProjectMarket = async (
    projectId: Uuid,
    signal?:   GenericAbortSignal,
): Promise<ProjectMarketGroup[]> => (await axiosClient())
    .get(
        `/api/projects/${projectId}/market`,
        {
            signal,
        }
    )
    .then(x => x.data);

export const useListProjectMarket = (
    projectId: Uuid,
) => {
    return useQuery({
        queryKey: [LIST_PROJECT_MARKET, projectId],
        queryFn: async ({
            signal
        }: AbortSignal) => listProjectMarket(projectId, signal),
        initialData: [],
    })
}

export type ProjectMarketGroup = {
    header:     string;
    entries:    ProjectMarketEntry[];
}

export type ProjectMarketEntry = {
    item:       Item;
    quantity:   number;

    cost?:      number;
    source?:    string;

    cost_multi: MarketBulkResponse;
    cost_smart: MarketBulkResponse[];
    cost_smart2: MarketBulkResponse[];
}

export type MarketBulkResponse = {
    source:             number;
    type_id:            number;
    quantity:           number;
    remaining:          number;
    price:              number;
    insufficient_data:  boolean;
}
