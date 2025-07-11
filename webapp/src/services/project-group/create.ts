import { axiosClient } from "@/services/client";
import type { Uuid } from "@/services/utils";

export const CREATE_PROJECT_GROUPS = 'createProjectGroup';

export const createProjectGroup = async (
    data: CreateProjectGroup,
): Promise<CreateProjectGroupResponse> => (await axiosClient())
    .post(
        '/api/project-groups',
        data,
    )
    .then(x => x.data);

export interface CreateProjectGroup {
    name: string;
    description?: string;
}

export interface CreateProjectGroupResponse {
    id: Uuid,
}
