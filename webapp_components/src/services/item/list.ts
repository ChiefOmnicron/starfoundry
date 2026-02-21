import { axiosClient, type AbortSignal } from "@internal/services/client";
import { useQuery } from "@tanstack/react-query";
import type { Item } from "@internal/services/item/model";
import type { GenericAbortSignal } from "axios";

export const LIST_ITEM = 'listItem';

export const listItem = async (
    filter:  ItemFilter,
    signal?: GenericAbortSignal,
): Promise<Item[]> => (await axiosClient())
    .get(
        `/api/items`,
        {
            params: filter,
            signal,
        }
    )
    .then(x => x.data);

export type ItemFilter = {
    blueprint?: boolean;
    buildable?: boolean;
    name: string;
}

// For general use
export const useListItems = (
    filter: ItemFilter,
) => {
    return useQuery(listItemQuery(filter));
}

// For pre-fetching
export const listItemQuery = (
    filter: ItemFilter,
) => ({
    queryKey: [LIST_ITEM, filter],
    queryFn: async ({
        signal
    }: AbortSignal) => listItem(filter, signal),
    // ms * s * m
    staleTime: 1000 * 60 * 5,
});
