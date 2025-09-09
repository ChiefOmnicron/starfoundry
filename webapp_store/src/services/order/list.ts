import { axiosClient } from "@/services/client";
import { useQuery } from "@tanstack/react-query";
import type { Uuid } from "@/services/utils";
import type { Item } from "@/services/product/list";
import type { OrderStatusType } from "@/components/OrderStatus";
import type { CharacterInfo } from "@/services/character";

export const LIST_ORDER = 'listOrder';

export const listOrder = async (): Promise<Order[]> => (await axiosClient())
    .get(
        `/api/orders`,
    )
    .then(x => x.data);

// For general use
export const useListOrders = () => {
    return useQuery(listOrderQuery());
}

// For pre-listing
export const listOrderQuery = () => ({
    queryKey: [LIST_ORDER],
    queryFn: async () => listOrder(),
    initialData: [],
});

export type Order = {
    id:                 Uuid,
    character:          CharacterInfo,
    quantity:           number,
    delivery_location:  string,
    status:             OrderStatusType,
    ordered_at:         string,
    comment:            string,

    products:           OrderProduct[],
}

export type OrderProduct = {
    name:           string,
    price:          number,
    image_type:     string,
    image_type_id:  number,
    content:        Item[],
    is_additional:  boolean,
}
