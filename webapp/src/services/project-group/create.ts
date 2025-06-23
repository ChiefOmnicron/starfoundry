import axios from "axios";
import type { Uuid } from "../utils";

export const CREATE_PROJECT_GROUPS = 'createProjectGroup';

export const createProjectGroup = async (
    data: CreateProjectGroup,
): Promise<CreateProjectGroupResponse> => axios.post(
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
