import type { TypeId, Uuid } from "../utils";
import { useQuery } from "@tanstack/react-query";

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
    id:                Uuid,
    structure_id:      number,
    name:              string,
    system:            StructureSystem,
    structure_type:    ItemTmp,
    rigs:              StructureRig[],
    services:          ItemTmp[],
}

export type StructureSystem = {
    region_id:          number,
    region_name:        string,
    constellation_id:   number,
    constellation_name: string,
    system_id:          number,
    system_name:        string,
    security:           string,
    security_group:     'HIGHSEC' | 'LOWSEC' | 'NULLSEC',
}

export type ItemTmp = {
    type_id:        number,
    category_id:    number,
    group_id:       number,
    meta_group_id?: number,
    base_price:     number,
    volume:         number,
    name:           string,
    repackaged?:    number,
}

export type StructureRig = {
    name:            string,
    type_id:         TypeId,

    material?:       number,
    time?:           number,
    category_groups: number[],
}

export type StructureFilter = {
    name?: string,
    system_id?: number,
    structure_type_id?: number,
    service_id?: number,
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
