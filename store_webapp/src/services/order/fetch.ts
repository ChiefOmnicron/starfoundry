import { axiosClient } from "@/services/client";
import { useQuery } from "@tanstack/react-query";
import type { Order } from "@/services/order/list";
import type { Uuid } from "@/services/utils";

export const FETCH_ORDER = 'fetchOrder';

export const fetchOrder = async (
    orderUuid: Uuid,
): Promise<Order> => (await axiosClient())
    .get(
        `/api/orders/${orderUuid}`,
    )
    .then(x => x.data);

// For general use
export const useFetchOrder = (
    orderUuid: Uuid,
) => {
    return useQuery(fetchOrderQuery(orderUuid));
}

// For pre-listing
export const fetchOrderQuery = (
    orderUuid: Uuid,
) => ({
    queryKey: [FETCH_ORDER, orderUuid],
    queryFn: async () => fetchOrder(orderUuid),
});
