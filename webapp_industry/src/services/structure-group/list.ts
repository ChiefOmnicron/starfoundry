import { axiosClient } from "@/services/client";
import { useQuery } from "@tanstack/react-query";
import type { Uuid } from "@/services/utils";
import type { Structure } from "@/services/structure/list";

export const LIST_STRUCTURE_GROUP = 'listStructureGroup';

export const listStructureGroup = async (
    filter: StructureGroupFilter,
): Promise<StructureGroup[]> => (await axiosClient())
    .get(
        `/api/structure-groups`,
        {
            params: filter,
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
    queryFn: async () => listStructureGroup(filter),
    // ms * s * m
    staleTime: 1000 * 60 * 5,
});
