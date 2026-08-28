import { axiosClient } from "../client";
import type { TypeId, Uuid } from "../utils";

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
