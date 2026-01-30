import { axiosClient, type CharacterInfo } from "@/services/client";
import type { GenericAbortSignal } from "axios";

export const LIST_ITEM = 'listItem';

export const inGameSearch = async (
    filter:  InGameSearchFilter,
    signal?: GenericAbortSignal,
): Promise<CharacterInfo[]> => (await axiosClient())
    .get(
        `/api/search`,
        {
            params: {
                search: filter.search,
                category: filter.categories.join(','),
            },
            signal,
        }
    )
    .then(x => x.data);

export type InGameSearchFilter = {
    search: string;
    categories: string[];
}
