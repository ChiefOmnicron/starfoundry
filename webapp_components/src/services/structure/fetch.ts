import { axiosClient, type AbortSignal } from "@internal/services/client";
import { useQuery } from "@tanstack/react-query";
import type { Structure } from "@internal/services/structure/list";
import type { Uuid } from "@internal/services/utils";
import type { GenericAbortSignal } from "axios";

export const FETCH_STRUCTURE = 'fetchStructure';

export const fetchStructure = async (
    structureId: Uuid,
    filter?:     FetchStructureQuery,
    signal?:     GenericAbortSignal,
): Promise<Structure> => (await axiosClient())
    .get(
        `/api/structures/${structureId}`,
        {
            params: filter,
            signal,
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
    queryFn: async ({
        signal
    }: AbortSignal) => fetchStructure(id, filter, signal),
    // ms * s * m
    staleTime: 1000 * 60 * 5,
});

export type FetchStructureQuery = {
    include_installable: boolean;
};
