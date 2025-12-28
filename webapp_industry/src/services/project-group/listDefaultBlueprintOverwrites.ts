import { axiosClient, type AbortSignal } from "@/services/client";
import { useQuery } from "@tanstack/react-query";
import type { Item } from "../item/model";
import type { Uuid } from "@/services/utils";
import type { GenericAbortSignal } from "axios";

export const LIST_PROJECT_GROUP_DEFAULT_BLUEPRINT_OVERWRITES = 'listProjectGroupsDefaultBlueprintOverwrites';

export const listProjectGroupDefaultBlueprintOverwrites = async (
    projectGroupUuid: Uuid,
    signal?:          GenericAbortSignal,
): Promise<BlueprintOverwrite[]> => (await axiosClient())
    .get(
        `/api/project-groups/${projectGroupUuid}/defaults/blueprint-overwrites`,
        {
            signal,
        }
    )
    .then(x => x.data);

export type BlueprintOverwrite = {
    item:                Item;
    material_efficiency: number;
}

// For general use
export const useListProjectGroupDefaultBlueprintOverwrites = (
    projectGroupUuid: Uuid,
) => {
    return useQuery(listProjectGroupDefaultBlueprintOverwritesQuery(projectGroupUuid));
}

// For pre-fetching
export const listProjectGroupDefaultBlueprintOverwritesQuery = (
    projectGroupUuid: Uuid,
) => ({
    queryKey: [LIST_PROJECT_GROUP_DEFAULT_BLUEPRINT_OVERWRITES, projectGroupUuid],
    queryFn: async ({
        signal
    }: AbortSignal) => listProjectGroupDefaultBlueprintOverwrites(projectGroupUuid, signal),
});
