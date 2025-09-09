import { axiosClient } from "@/services/client";
import { useQuery } from "@tanstack/react-query";
import type { TypeId, Uuid } from "@/services/utils";
import type { Item } from "@/services/item/model";

export const LIST_STRUCTURE = 'listStructure';

export const listStructure = async (
    filter: StructureFilter,
): Promise<Structure[]> => (await axiosClient())
    .get(
        `/api/structures`,
        {
            params: filter,
        }
    )
    .then(x => x.data);

export type Structure = {
    id:           Uuid;
    structure_id: number;
    name:         string;
    system:       StructureSystem;
    structure:    Item;
    rigs:         StructureRig[];
    services:     Item[];
}

export type StructureSystem = {
    region_id:          number;
    region_name:        string;
    constellation_id:   number;
    constellation_name: string;
    system_id:          number;
    system_name:        string;
    security:           string;
    security_group:     'HIGHSEC' | 'LOWSEC' | 'NULLSEC';
}

export type StructureRig = {
    type_id:         TypeId;
    item:            Item;

    material?:       number;
    time?:           number;
    category_groups: number[];
}

export type StructureFilter = {
    name?: string;
    system_id?: number;
    structure_type_id?: number;
    service_id?: number;
}

// For general use
export const useListStructure = (
    filter: StructureFilter,
) => {
    return useQuery(listStructureQuery(filter));
}

// For pre-listing
export const listStructureQuery = (
    filter: StructureFilter,
) => ({
    queryKey: [LIST_STRUCTURE, filter],
    queryFn: async () => listStructure(filter),
    initialData: [],
});
