import { axiosClient } from "@internal/services/client";
import type { Item } from "@internal/services/item/model";
import type { Uuid } from "@internal/services/utils";

export const LIST_PROJECT = 'listProject';

export const splitJobCheck = async (
    projectId:  Uuid,
    data:       SplitJobRequest,
): Promise<SplitJobResponse> => (await axiosClient())
    .put(
        `/api/projects/${projectId}/split-job/check`,
        data,
    )
    .then(x => {
        return x.data;
    });

export type SplitJobRequest = {
    old: SplitJobEntry,
    new: SplitJobEntry[],
}

export type SplitJobEntry = {
    runs:       number;
    type_id:    number;
}

export type SplitJobResponse = {
    excess:     SplitJobResponseMarketEntry[];
    jobs:       SplitJobResponseJobEntry[];
    materials:  SplitJobResponseMarketEntry[];
}

export type SplitJobResponseJobEntry = {
    item:           Item,
    runs:           number,
    structure_id:   Uuid,
}

export type SplitJobResponseMarketEntry = {
    item:       Item,
    quantity:   number,
}
