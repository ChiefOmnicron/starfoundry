import { axiosClient, type AbortSignal } from "@internal/services/client";
import { useQuery } from "@tanstack/react-query";
import type { GenericAbortSignal } from "axios";
import type { Uuid } from "@internal/services/utils";
import type { Item } from "@internal/services/item/model";

export const LIST_PROJECT_MARKET = 'listProjectMarket';

export const listProjectMarketBuy = async (
    projectId: Uuid,
    query:     ProjectMarketBuyQuery,
    signal?:   GenericAbortSignal,
): Promise<ProjectMarketBuyEntry[]> => (await axiosClient())
    .get(
        `/api/projects/${projectId}/market/buy`,
        {
            signal,
            params: query,
        }
    )
    .then(x => x.data);

export const useListProjectMarketBuy = (
    projectId: Uuid,
    query:     ProjectMarketBuyQuery,
) => {
    return useQuery({
        queryKey: [LIST_PROJECT_MARKET, projectId, query.strategy],
        queryFn: async ({
            signal
        }: AbortSignal) => listProjectMarketBuy(projectId, query, signal),
        initialData: [],
    })
}

export type ProjectMarketBuyQuery = {
    strategy: 'MULTI_BUY' | 'SMART_BUY';
}

export type ProjectMarketBuyEntry = {
    item:       Item;
    quantity:   number;

    cost?:      number;
    source?:    string;

    entries:    MarketBulkResponse[];
}

export type MarketBulkResponse = {
    source:             number;
    type_id:            number;
    quantity:           number;
    remaining:          number;
    price:              number;
    insufficient_data:  boolean;
    last_fetch:         string;
}
