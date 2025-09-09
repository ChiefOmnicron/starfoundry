import type { TypeId, Uuid } from "../utils";
import { useQuery } from "@tanstack/react-query";
import { axiosClient } from "../client";
import type { AdditionalOption } from "@/routes/admin/-components/productForm";

export const LIST_PRODUCT = 'listProduct';

export const listProduct = async (

): Promise<Product[]> => (await axiosClient())
    .get(
        `/api/products`,
    )
    .then(x => x.data);

// For general use
export const useListProducts = () => {
    return useQuery(listProductQuery());
}

// For pre-listing
export const listProductQuery = () => ({
    queryKey: [LIST_PRODUCT],
    queryFn: async () => listProduct(),
    initialData: [],
});

export type Product = {
    uuid:               Uuid,
    name:               string,
    description?:       string,
    price:              number,
    category:           string,
    options:            ProductOption[],
    allowed_stations:   TypeId[],
    whitelist:          number[],
    tags:               string[],
    image_type:         'render' | 'icon',
    image_type_id:      number,
    content:            Item[],
    additional_options: AdditionalOption[],
}

export type Item = {
    name: string,
    quantity: number,
    type_id: number,
}

export type ProductOption = {
    option_type: OptionType,

    uuid?:       Uuid,

    name?:       string,
    deduction?:  number,
}

export type OptionType = 'OPTION' | 'OFFER';
