import { axiosClient } from "@/services/client";
import type { Uuid } from "@/services/utils";

export const UPDATE_ORDER = 'updateOrder';

export const updateOrderComment = async (
    orderId: Uuid,
    data:    UpdateOrder,
): Promise<void> => (await axiosClient())
    .put(
        `/api/orders/${orderId}`,
        data,
    );

export type UpdateOrder = {
    comment: string,
}
