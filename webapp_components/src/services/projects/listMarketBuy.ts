import { axiosClient, type AbortSignal } from "@internal/services/client";
import { useQuery } from "@tanstack/react-query";
import type { GenericAbortSignal } from "axios";
import type { Uuid } from "@internal/services/utils";
import type { Item } from "@internal/services/item/model";

export const LIST_PROJECT_MARKET = 'listProjectMarket';

export const listProjectMarketBuy = async (
    projectId: Uuid,
    config:    ProjectMarketBuyQuery,
    signal?:   GenericAbortSignal,
): Promise<ProjectMarketBuyEntry[]> => (await axiosClient())
    .post(
        `/api/projects/${projectId}/market/buy`,
        config,
        {
            signal,
        }
    )
    .then(x => x.data);

export const useListProjectMarketBuy = (
    projectId: Uuid,
    config:    ProjectMarketBuyQuery,
) => {
    return useQuery({
        queryKey: [LIST_PROJECT_MARKET, projectId, config],
        queryFn: async ({
            signal
        }: AbortSignal) => listProjectMarketBuy(projectId, config, signal),
        initialData: [],
    })
}

export type ProjectMarketBuyQuery = {
    strategy: 'MULTI_BUY' | 'SMART_BUY';

    structure_ids: number[];
    virtual_market: boolean;

    gas_decompression?: string;
    mineral_compression?: string;
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
