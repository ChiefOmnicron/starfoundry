import { axiosClient, type AbortSignal } from "@/services/client";
import { useQuery } from "@tanstack/react-query";
import type { GenericAbortSignal } from "axios";
import type { Structure } from "@/services/structure/list";
import type { Uuid } from "@/services/utils";

export const LIST_STRUCTURE_GROUP = 'listStructureGroup';

export const listStructureGroup = async (
    filter:  StructureGroupFilter,
    signal?: GenericAbortSignal,
): Promise<StructureGroup[]> => (await axiosClient())
    .get(
        `/api/structure-groups`,
        {
            params: filter,
            signal,
        }
    )
    .then(x => x.data);

export type StructureGroup = {
    id:         Uuid;
    name:       string;
    structures: Structure[],
}

export type StructureGroupFilter = {
    name?: string;
}

// For general use
export const useListStructureGroup = (
    filter: StructureGroupFilter,
) => {
    return useQuery(listStructureGroupQuery(filter));
}

// For pre-loading
export const listStructureGroupQuery = (
    filter: StructureGroupFilter,
) => ({
    queryKey: [LIST_STRUCTURE_GROUP, filter],
    queryFn: async ({
        signal,
    }: AbortSignal) => listStructureGroup(filter, signal),
    // ms * s * m
    staleTime: 1000 * 60 * 5,
});
