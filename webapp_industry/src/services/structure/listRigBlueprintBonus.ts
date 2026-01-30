import { axiosClient, type AbortSignal } from "@/services/client";
import { useQuery } from "@tanstack/react-query";
import type { GenericAbortSignal } from "axios";
import type { Item } from "@/services/item/model";

export const FETCH_RIG_STRUCTURE_BONUS = "listRigBlueprintBonus";

export const listRigBlueprintBonus = async (
    rigTypeIds: number[],
    signal?:    GenericAbortSignal,
): Promise<RigBlueprintBonus[]> =>
    (await axiosClient())
        .post(
            `/api/eve/structures/rigs/blueprints`,
            rigTypeIds,
            {
                signal,
            }
        )
        .then((x) => x.data);

// For general use
export const useListRigBlueprintBonus = (
    rigTypeIds: number[],
) => {
    return useQuery(listRigBlueprintBonusQuery(rigTypeIds));
}

// For pre-fetching
export const listRigBlueprintBonusQuery = (
    rigTypeIds: number[],
) => ({
    queryKey: [FETCH_RIG_STRUCTURE_BONUS, rigTypeIds],
    queryFn: async ({
        signal
    }: AbortSignal) => listRigBlueprintBonus(rigTypeIds, signal),
    // ms * s * m * h -> 24 hours
    staleTime: 1000 * 60 * 60 * 24,
    refetchOnWindowFocus: false,
});

export type RigBlueprintBonus = {
    bonus_me:         number;
    bonus_te:         number;

    is_manufacturing: boolean;
    is_reaction:      boolean;

    blueprint:        Item;
}
