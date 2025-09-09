import type { ProductForm } from "@/routes/admin/-components/productForm";
import { axiosClient } from "@/services/client";
import type { Uuid } from "@/services/utils";

export const CREATE_PRODUCT = 'createProduct';

export const createProduct = async (
    data: ProductForm,
): Promise<CreateProductResponse> => (await axiosClient())
    .post(
        '/api/products',
        data,
    )
    .then(x => x.data);

export type CreateProductResponse = {
    id: Uuid,
}
