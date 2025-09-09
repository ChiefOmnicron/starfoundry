import { axiosClient } from "../client";
import { useQuery } from "@tanstack/react-query";
import type { Product } from "./list";
import type { Uuid } from "../utils";

export const FETCH_PRODUCT = 'fetchProduct';

export const fetchProduct = async (
    productUuid: Uuid,
): Promise<Product> => (await axiosClient())
    .get(
        `/api/products/${productUuid}`,
    )
    .then(x => x.data);

// For general use
export const useFetchProduct = (
    productUuid: Uuid,
) => {
    return useQuery(fetchProductQuery(productUuid));
}

// For pre-listing
export const fetchProductQuery = (
    productUuid: Uuid,
) => ({
    queryKey: [FETCH_PRODUCT],
    queryFn: async () => fetchProduct(productUuid),
});
