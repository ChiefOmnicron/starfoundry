import { axiosClient, type AbortSignal } from "@internal/services/client";
import { useQuery } from "@tanstack/react-query";
import type { GenericAbortSignal } from "axios";
import type { Uuid } from "@internal/services/utils";

export const LIST_TAGS = 'listTags';

export const listTags = async (
    filter: ListTagFilter,
    signal?: GenericAbortSignal,
): Promise<Tag[]> => (await axiosClient())
    .get(
        '/api/tags',
        {
            params: filter,
            signal,
        }
    )
    .then(x => {
        if (x.status === 204) {
            return [];
        }

        return x.data;
    });

export const useListTags = (filter: ListTagFilter) => {
    return useQuery({
        queryKey: [LIST_TAGS],
        queryFn: async ({
            signal
        }: AbortSignal) => listTags(filter, signal),
    })
}

export type TagType = 'MANUAL' | 'AUTO';
export type AutoTagSelect = 'PROJECT_NAME' | 'PROJECT_ORDERER' | 'PROJECT_NOTE' | 'PROJECT_PRODUCT' | 'PROJECT_STATUS';
export type AutoTagCompare = 'IS' | 'IS_NOT' | 'CONTAINS' | 'PATTERN';

export type Tag = {
    id:         Uuid;
    color:      string;
    content:    string;
    typ:        TagType;

    auto:       TagAuto[];
}

export type TagAuto = {
    select:     AutoTagSelect;
    compare:    AutoTagCompare;
    value:      string;
}

export type ListTagFilter = {
    auto: boolean,
    manual: boolean,
}
