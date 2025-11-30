import { axiosClient } from "@/services/client";
import { useQuery } from "@tanstack/react-query";
import type { Structure } from "@/services/structure/list";
import type { Uuid } from "@/services/utils";

export const FETCH_STRUCTURE = 'fetchStructure';

export const fetchStructure = async (
    structureId: Uuid,
    filter?:     FetchStructureQuery,
): Promise<Structure> => (await axiosClient())
    .get(
        `/api/structures/${structureId}`,
        {
            params: filter,
        }
    )
    .then(x => x.data);

// For general use
export const useFetchStructure = (
    id:      Uuid,
    filter?: FetchStructureQuery,
) => {
    return useQuery(fetchStructureQuery(id, filter));
}

// For pre-fetching
export const fetchStructureQuery = (
    id:      Uuid,
    filter?: FetchStructureQuery,
) => ({
    queryKey: [FETCH_STRUCTURE, id, filter],
    queryFn: async () => fetchStructure(id, filter),
    // ms * s * m
    staleTime: 1000 * 60 * 5,
});

export type FetchStructureQuery = {
    include_installable: boolean;
};
