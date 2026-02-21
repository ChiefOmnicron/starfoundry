import { axiosClient } from "@internal/services/client";
import type { Category } from "@internal/services/utils";
import type { GenericAbortSignal } from "axios";

export const LIST_ITEM = 'listItem';

export const inGameSearch = async (
    filter:  InGameSearchFilter,
    signal?: GenericAbortSignal,
): Promise<InGameSearchResponse[]> => (await axiosClient())
    .get(
        `/api/search`,
        {
            params: {
                search: filter.search,
                categories: filter.categories.join(','),
            },
            signal,
        }
    )
    .then(x => x.data);

export type InGameSearchFilter = {
    search:     string;
    categories: Category[];
}

export type InGameSearchResponse = {
    id:       number;
    category: Category,
    name:     string;
}
