import { axiosClient } from "@/services/client";
import { useQuery } from "@tanstack/react-query";
import type { Order } from "@/services/order/list";

export const LIST_ORDER_ADMIN = 'listOrderAdmin';

export const listOrder = async (

): Promise<Order[]> => (await axiosClient())
    .get(
        `/api/admin/orders`,
    )
    .then(x => x.data);

// For general use
export const useListOrdersAdmin = () => {
    return useQuery(listOrderAdminQuery());
}

// For pre-listing
export const listOrderAdminQuery = () => ({
    queryKey: [LIST_ORDER_ADMIN],
    queryFn: async () => listOrder(),
    initialData: [],
});
