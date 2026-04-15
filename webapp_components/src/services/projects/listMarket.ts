import { axiosClient, type AbortSignal } from "@internal/services/client";
import { useQuery } from "@tanstack/react-query";
import type { GenericAbortSignal } from "axios";
import type { Uuid } from "@internal/services/utils";
import type { Item } from "@internal/services/item/model";

export const LIST_PROJECT_MARKET = 'listProjectMarket';

export const listProjectMarket = async (
    projectId: Uuid,
    signal?:   GenericAbortSignal,
): Promise<ProjectMarketEntry[]> => (await axiosClient())
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

export type ProjectMarketEntry = {
    id:         Uuid;
    item:       Item;
    quantity:   number;

    cost?:      number;
    source?:    string;
}
