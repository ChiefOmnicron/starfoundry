import { axiosClient } from "../client";
import { useQuery } from "@tanstack/react-query";
import type { Uuid } from "../utils";
import type { Item, Product } from "../product/list";

export const LIST_ORDER = 'listOrder';

export const listOrder = async (

): Promise<Order[]> => (await axiosClient())
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
    uuid:               Uuid,
    quantity:           number,
    delivery_location:  string,
    status:             string,
    ordered_at:         string,

    products:           Product[],
}

export type OrderProduct = {
    name:           string,
    price:          number,
    image_type:     string,
    image_type_id:  number,
    content:        Item[],
    is_additional:  boolean,
}
