import { axiosClient } from "../axiosClient";
import type { Uuid } from "../utils";

export const deleteFilter = async (
    filterId: Uuid,
): Promise<void> => (await axiosClient())
    .delete(
        `/api/filters/${filterId}`,
    )
    .then(x => x.data);
