import { axiosClient } from "../axiosClient";
import type { Uuid } from "../utils";

export const deleteTag = async (
    tagId: Uuid,
): Promise<void> => (await axiosClient())
    .delete(
        `/api/tags/${tagId}`,
    )
    .then(x => x.data);
