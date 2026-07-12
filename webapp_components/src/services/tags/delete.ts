import { axiosClient } from "@internal/services/client";
import type { Uuid } from "@internal/services/utils";

export const deleteTag = async (
    tagId: Uuid,
): Promise<void> => (await axiosClient())
    .delete(
        `/api/tags/${tagId}`,
    )
    .then(x => x.data);
