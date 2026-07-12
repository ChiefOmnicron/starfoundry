import { axiosClient } from "@internal/services/client";
import type { Uuid } from "@internal/services/utils";
import type { CreateTag } from "@internal/services/tags/create";

export const updateTag = async (
    id: Uuid,
    data: CreateTag,
): Promise<UpdateTagResponse> => (await axiosClient())
    .put(
        `/api/tags/${id}`,
        data,
    )
    .then(x => x.data);

export type UpdateTagResponse = {
    id: Uuid,
}
