import { axiosClient } from "@/services/client";
import { useQuery } from "@tanstack/react-query";
import type { Order } from "@/services/order/list";
import type { Uuid } from "@/services/utils";

export const FETCH_ORDER_ADMIN = 'fetchOrderAdmin';

export const fetchOrderAdmin = async (
    orderUuid: Uuid,
): Promise<Order> => (await axiosClient())
    .get(
        `/api/admin/orders/${orderUuid}`,
    )
    .then(x => x.data);

// For general use
export const useFetchOrderAdmin = (
    orderUuid: Uuid,
) => {
    return useQuery(fetchOrderAdminQuery(orderUuid));
}

// For pre-listing
export const fetchOrderAdminQuery = (
    orderUuid: Uuid,
) => ({
    queryKey: [FETCH_ORDER_ADMIN],
    queryFn: async () => fetchOrderAdmin(orderUuid),
});
