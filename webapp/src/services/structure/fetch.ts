import axios from "axios";
import type { TypeId, Uuid } from "../utils";
import { useQuery } from "@tanstack/react-query";

export const FETCH_STRUCTURE = 'fetchStructure';

export const fetchStructure = async (
    filter: StructureFilter,
): Promise<Structure[]> => axios.get(
        `/api/structures`,
        {
            params: filter,
        }
    )
    .then(x => x.data);

export type Structure = {
    id:                Uuid,
    name:              string,
    system_id:         number,
    security:          'HIGHSEC' | 'LOWSEC' | 'NULLSEC',
    structure_type:    TypeId,
    rigs:              StructureRig[],
    services:          TypeId[],
    structure_id:      number,
}

export type StructureRig = {
    name:            string,
    type_id:         TypeId,

    material?:       number,
    time?:           number,
    category_groups: number[],
}

export type StructureFilter = {

}

// For general use
export const useFetchStructure = (
    filter: StructureFilter,
) => {
    return useQuery(fetchStructureQuery(filter));
}

// For pre-fetching
export const fetchStructureQuery = (
    filter: StructureFilter,
) => ({
    queryKey: [FETCH_STRUCTURE, filter],
    queryFn: async () => fetchStructure(filter),
});
