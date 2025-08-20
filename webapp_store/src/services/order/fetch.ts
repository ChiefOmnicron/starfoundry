import { axiosClient } from "../client";
import { useQuery } from "@tanstack/react-query";
import type { Order } from "./list";
import type { Uuid } from "../utils";

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
    queryKey: [FETCH_ORDER],
    queryFn: async () => fetchOrder(orderUuid),
});
