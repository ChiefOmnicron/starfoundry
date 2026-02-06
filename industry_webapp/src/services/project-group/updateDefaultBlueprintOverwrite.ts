import { axiosClient } from "@/services/client";
import type { TypeId, Uuid } from "@/services/utils";

export const updateDefaultBlueprintOverwrite = async (
    projectGroupUuid:   Uuid,
    blueprintOverwrite: UpdateBlueprintOverwrite[],
): Promise<void> => (await axiosClient())
    .put(
        `/api/project-groups/${projectGroupUuid}/defaults/blueprint-overwrites`,
        blueprintOverwrite,
    );

export type UpdateBlueprintOverwrite = {
    type_id:             TypeId;
    material_efficiency: number;
}
