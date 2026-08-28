import { axiosClient } from "../client";
import type { Uuid } from "../utils";
import type { CreateTag } from "../tags/create";

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
