import { axiosClient } from "../client";
import type { Uuid } from "../utils";

export const CREATE_PROJECT_GROUPS = 'createProjectGroup';

export const createProjectGroup = async (
    data: CreateProjectGroup,
): Promise<CreateProjectGroupResponse> => (await axiosClient())
    .post(
        '/api/project-groups',
        data,
    )
    .then(x => x.data);

export type CreateProjectGroup = {
    name: string;
    description?: string;
}

export type CreateProjectGroupResponse = {
    id: Uuid,
}
