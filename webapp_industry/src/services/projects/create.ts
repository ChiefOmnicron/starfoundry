import { axiosClient } from "@/services/client";
import type { Uuid } from "@/services/utils";

export const createProject = async (
    data: CreateProject,
): Promise<CreateProjectResponse> => (await axiosClient())
    .post(
        '/api/projects',
        data,
    )
    .then(x => x.data);

export type CreateProject = {
    sell_price?:      number;
    project_group_id: Uuid;
    orderer:          string;
    name:             string;
}

export type CreateProjectResponse = {
    id: Uuid,
}
