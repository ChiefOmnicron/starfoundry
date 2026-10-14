import { axiosClient, type AbortSignal } from "../axiosClient";
import { useQuery } from "@tanstack/react-query";
import type { GenericAbortSignal } from "axios";
import type { Uuid } from "../utils";

export const LIST_FILTERS = 'listFilters';

export const listFilters = async (
    filter:  FilterQuery,
    signal?: GenericAbortSignal,
): Promise<Filter[]> => (await axiosClient())
    .get(
        '/api/filters',
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

export type FilterQuery = {
    typ?: FilterType;
}

export const useListFilters = (
    filterParams: FilterQuery,
) => {
    return useQuery({
        queryKey: [LIST_FILTERS, filterParams],
        queryFn: async ({
            signal
        }: AbortSignal) => listFilters(filterParams, signal),
        // 10 minutes (ms * s * m)
        staleTime: 1000 * 60 * 10,
    })
}

export type Filter = {
    id:         Uuid;
    name:       string;
    filters:    any;
}

export type FilterType = 'PROJECT_JOB';
