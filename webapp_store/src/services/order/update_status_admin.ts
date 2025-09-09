import { axiosClient } from "@/services/client";
import type { Uuid } from "@/services/utils";

export const LIST_ORDER_ADMIN = 'updateOrderStatusAdmin';

export const updateOrderStatusAdmin = async (
    orderId: Uuid,
    status:  'IN_PROGRESS' | 'DELIVERED',
): Promise<void> => (await axiosClient())
    .put(
        `/api/admin/orders/${orderId}/status`,
        {
            status,
        }
    );
