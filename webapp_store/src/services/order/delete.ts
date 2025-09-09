import { axiosClient } from "@/services/client";
import type { Uuid } from "@/services/utils";

export const deleteOrder = async (
    orderUuid: Uuid,
): Promise<void> => (await axiosClient())
    .delete(
        `/api/orders/${orderUuid}`,
    );
