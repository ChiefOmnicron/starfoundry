import { axiosClient, type AbortSignal } from "../client";
import { useQuery } from "@tanstack/react-query";
import type { GenericAbortSignal } from "axios";
import type { Structure } from "../structure/list";
import type { Uuid } from "../utils";

export const LIST_PROJECT_MARKET_STRUCTURE = 'listProjectMarketStructure';

export const listProjectMarketStructures = async (
    projectId: Uuid,
    signal?:   GenericAbortSignal,
): Promise<Structure[]> => (await axiosClient())
    .get(
        `/api/projects/${projectId}/market/structures`,
        {
            signal,
        }
    )
    .then(x => x.data);

export const useListProjectMarketStructures = (
    projectId: Uuid,
) => {
    return useQuery({
        queryKey: [LIST_PROJECT_MARKET_STRUCTURE, projectId],
        queryFn: async ({
            signal
        }: AbortSignal) => listProjectMarketStructures(projectId, signal),
    })
}
