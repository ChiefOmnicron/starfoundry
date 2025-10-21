import { axiosClient } from "@/services/client";
import { useQuery } from "@tanstack/react-query";
import type { CharacterInfo } from "@/services/character";
import type { OrderProduct } from "@/services/order/list";
import type { OrderStatusType } from "@/components/OrderStatus";
import type { Uuid } from "@/services/utils";

export const LIST_ORDER_ADMIN = 'listOrderAdmin';

export const listOrder = async (

): Promise<AdminOrder[]> => (await axiosClient())
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

export type AdminOrder = {
    id:                      Uuid,
    character:               CharacterInfo,
    quantity:                number,
    delivery_location:       string,
    status:                  OrderStatusType,
    ordered_at:              string,
    comment:                 string,
    sf_industry_link?:       string,
    expected_delivery_date?: string,

    products:                OrderProduct[],
}
