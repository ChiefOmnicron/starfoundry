import { axiosClient, type AbortSignal } from "@internal/services/client";
import { useQuery } from "@tanstack/react-query";
import type { StructureRig } from "./list";
import type { GenericAbortSignal } from "axios";

export const FETCH_STRUCTURE_RIG = "fetchStructureRigs";

export const fetchStructureRigs = async (
    structureTypeId: number,
    signal?:         GenericAbortSignal,
): Promise<StructureRig[]> =>
    (await axiosClient())
        .get(
            `/api/eve/structures/rigs/${structureTypeId}`,
            {
                signal,
            }
        )
        .then((x) => x.data);

// For general use
export const useFetchStructureRigs = (
    structureTypeId: number,
) => {
    return useQuery(fetchStructureRigsQuery(structureTypeId));
}

// For pre-fetching
export const fetchStructureRigsQuery = (
    structureTypeId: number,
) => ({
    queryKey: [FETCH_STRUCTURE_RIG, structureTypeId],
    queryFn: async ({
        signal
    }: AbortSignal) => fetchStructureRigs(structureTypeId, signal),
    // ms * s * m
    staleTime: 1000 * 60 * 60,
});
