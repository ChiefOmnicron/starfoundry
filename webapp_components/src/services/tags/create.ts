import { axiosClient } from "@internal/services/client";
import type { Uuid } from "@internal/services/utils";
import type { TagType, AutoTagSelect, AutoTagCompare } from '@internal/services/tags/list';

export const createTag = async (
    data: CreateTag,
): Promise<CreateTagResponse> => (await axiosClient())
    .post(
        '/api/tags',
        data,
    )
    .then(x => x.data);

export type CreateTag = {
    color:      string;
    content:    string;
    typ:        TagType;

    auto?:      CreateTagAuto[];
}

export type CreateTagAuto = {
    select:     AutoTagSelect;
    compare:    AutoTagCompare;
    value:      string;
}

export type CreateTagResponse = {
    id: Uuid,
}
