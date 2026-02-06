import type { TypeId, Uuid } from "@/services/utils";
import { useQuery } from "@tanstack/react-query";
import { axiosClient } from "@/services/client";
import type { AdditionalOption } from "@/routes/admin/-components/productForm";
import type { FilterPropEntry } from "@/components/Filter";

export const LIST_PRODUCT = 'listProduct';
export const LIST_PRODUCT_TAGS = 'listProductTags';

export const listProduct = async (
    filter: ProductFilter,
): Promise<Product[]> => (await axiosClient())
    .get(
        `/api/products`,
        {
            params: filter,
        }
    )
    .then(x => x.data);

export const listProductTags = async (
    filter: ProductFilter,
): Promise<FilterPropEntry[]> => (await axiosClient())
    .get(
        `/api/products/tags`,
        {
            params: filter,
        }
    )
    .then(x => x.data);

// For general use
export const useListProducts = (filter: ProductFilter) => {
    return useQuery(listProductQuery(filter));
}

// For general use
export const useListProductTags = (filter: ProductFilter) => {
    return useQuery(listProductTagsQuery(filter));
}

// For pre-listing
export const listProductQuery = (filter: ProductFilter) => ({
    queryKey: [LIST_PRODUCT, filter],
    queryFn: async () => listProduct(filter),
    initialData: [],
});

// For pre-listing
export const listProductTagsQuery = (filter: ProductFilter) => ({
    queryKey: [LIST_PRODUCT_TAGS, filter],
    queryFn: async () => listProductTags(filter),
    initialData: [],
});

export type Product = {
    id:                 Uuid,
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
    delivery_time:      string,
    message?:           string,
    additional_options: AdditionalOption[],
    delivery_location:  number[],
    hidden:             boolean,
}

export type Item = {
    name: string,
    quantity: number,
    type_id: number,
}

export type ProductOption = {
    option_type: OptionType,

    id?:         Uuid,

    name?:       string,
    deduction?:  number,
}

export type OptionType = 'OPTION' | 'OFFER';

export type ProductFilter = {
    name?: string;
    category?: string;
    tags?: string;
}
