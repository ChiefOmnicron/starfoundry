import { useQuery } from "@tanstack/react-query";
import { axiosClient } from "@/services/client";

export const LIST_CATEGORIES = 'listCategory';

export const listCategory = async (

): Promise<string[]> => (await axiosClient())
    .get(
        `/api/products/categories`,
    )
    .then(x => x.data);

// For general use
export const useListCategory = () => {
    return useQuery(listCategoryQuery());
}

// For pre-listing
export const listCategoryQuery = () => ({
    queryKey: [LIST_CATEGORIES],
    queryFn: async () => listCategory(),
    initialData: [],
});
