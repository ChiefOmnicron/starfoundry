import { axiosClient } from "../axiosClient";
import type { Uuid } from "../utils";
import type { FilterType } from "./list";

export const createFilter = async (
    data: CreateFilter,
): Promise<CreateFilterResponse> => (await axiosClient())
    .post(
        '/api/filters',
        data,
    )
    .then(x => x.data);

export type CreateFilter = {
    name:       string;
    filters:    any;
    typ:        FilterType;
}

export type CreateFilterResponse = {
    id: Uuid,
}
