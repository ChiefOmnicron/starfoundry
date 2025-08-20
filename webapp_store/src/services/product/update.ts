import type { ProductForm } from "@/routes/admin/-components/productForm";
import { axiosClient } from "@/services/client";
import type { Uuid } from "@/services/utils";

export const UPDATE_PRODUCT = 'updateProduct';

export const updateProduct = async (
    uuid: Uuid,
    data: ProductForm,
): Promise<UpdateProductResponse> => (await axiosClient())
    .put(
        `/api/products/${uuid}`,
        data,
    )
    .then(x => x.data);

export type UpdateProductResponse = {
    id: Uuid,
}
