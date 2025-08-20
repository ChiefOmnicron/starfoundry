import type { OrderProduct } from "@/routes/products_/$productUuid.index";
import { axiosClient } from "@/services/client";
import type { Uuid } from "@/services/utils";

export const CREATE_PRODUCT = 'createOrder';

export const createOrder = async (
    productUuid: Uuid,
    data: OrderProduct,
): Promise<CreateOrderResponse> => (await axiosClient())
    .post(
        '/api/orders',
        {
            product_uuid: productUuid,
            quantity: data.quantity,
            delivery_location: data.deliverySystem,
            additional_option: data.additionalOptions === '00000000-0000-0000-0000-000000000000' ? null : data.additionalOptions,
            comment: data.comment,
        },
    )
    .then(x => x.data);

export type CreateOrderResponse = {
    id: Uuid,
}

export type CreateOrder = {
    product_uuid:       Uuid,
    quantity:           number,
    delivery_location:  string,
    additional_option?: Uuid,
    comment?:           string,
}
