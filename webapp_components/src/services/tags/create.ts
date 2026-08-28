import { axiosClient } from "../client";
import type { Uuid } from "../utils";
import type { TagType, AutoTagSelect, AutoTagCompare } from '../tags/list';

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
