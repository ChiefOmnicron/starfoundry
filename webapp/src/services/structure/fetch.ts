import { axiosClient } from "@/services/client";
import { useQuery } from "@tanstack/react-query";
import type { Structure } from "./list";
import type { Uuid } from "@/services/utils";

export const FETCH_STRUCTURE = 'fetchStructure';

export const fetchStructure = async (
    id: Uuid,
): Promise<Structure> => (await axiosClient())
    .get(
        `/api/structures/${id}`,
    )
    .then(x => x.data);

// For general use
export const useFetchStructure = (
    id: Uuid,
) => {
    return useQuery(fetchStructureQuery(id));
}

// For pre-listing
export const fetchStructureQuery = (
    id: Uuid,
) => ({
    queryKey: [FETCH_STRUCTURE, id],
    queryFn: async () => fetchStructure(id),
});
