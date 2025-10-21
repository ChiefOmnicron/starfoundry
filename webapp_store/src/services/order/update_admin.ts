import type { OrderStatusType } from "@/components/OrderStatus";
import { axiosClient } from "@/services/client";
import type { Uuid } from "@/services/utils";

export const LIST_ORDER_ADMIN = 'updateOrderStatusAdmin';

export const updateOrderAdmin = async (
    orderId: Uuid,
    data: UpdateOrderAdmin,
): Promise<void> => (await axiosClient())
    .put(
        `/api/admin/orders/${orderId}`,
        data,
    );

export type UpdateOrderAdmin = {
    status:                  OrderStatusType;
    expected_delivery_date?: Date;
    sf_industry_link?:       string;
}
