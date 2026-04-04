import { axiosClient } from "@internal/services/client";
import type { Uuid } from "@internal/services/utils";

export const updateMisc = async (
    projectId:  Uuid,
    data:       UpdateMiscRequest[],
): Promise<void> => (await axiosClient())
    .put(
        `/api/projects/${projectId}/misc`,
        data,
    )
    .then(x => {
        return x.data;
    });

export type UpdateMiscRequest = {
    item:           string;
    cost:           number;

    description?:   string;
    quantity?:      number;
}
