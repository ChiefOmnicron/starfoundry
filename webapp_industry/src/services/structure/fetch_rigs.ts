import { axiosClient } from "@/services/client";
import { useQuery } from "@tanstack/react-query";
import type { StructureRig } from "./list";

export const FETCH_STRUCTURE_RIG = "fetchStructureRigs";

export const fetchStructureRigs = async (
    structureTypeId: number,
): Promise<StructureRig[]> =>
    (await axiosClient())
        .get(`/api/eve/structures/rigs/${structureTypeId}`)
        .then((x) => x.data);

// For general use
export const useFetchStructureRigs = (
    structureTypeId: number,
) => {
    return useQuery(fetchStructureRigsQuery(structureTypeId));
}

// For pre-listing
export const fetchStructureRigsQuery = (
    structureTypeId: number,
) => ({
    queryKey: [FETCH_STRUCTURE_RIG, structureTypeId],
    queryFn: async () => fetchStructureRigs(structureTypeId),
    // ms * s * m
    staleTime: 1000 * 60 * 60,
});
