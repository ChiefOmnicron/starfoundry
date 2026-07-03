import { axiosClient } from "@internal/services/client";
import type { Uuid } from "@internal/services/utils";

export const createTag = async (
    data: CreateTag,
): Promise<CreateTagResponse> => (await axiosClient())
    .post(
        '/api/tags',
        data,
    )
    .then(x => x.data);

export type CreateTag = {
    color:          string;
    content:        string;
    typ:            TagType;

    auto?: {
        select:     AutoTagSelect;
        compare:    AutoTagCompare;
        value:      string;
    }
}

export type CreateTagResponse = {
    id: Uuid,
}

export type TagType = 'MANUAL' | 'AUTO';
export type AutoTagSelect = 'PROJECT_NAME' | 'PROJECT_ORDERER' | 'PROJECT_NOTE' | 'PROJECT_PRODUCT' | 'PROJECT_STATUS';
export type AutoTagCompare = 'IS' | 'IS_NOT' | 'CONTAINS' | 'PATTERN';
