import axios from "axios";
import type { Uuid } from "../utils";
import type { ProjectGroup } from "./fetch";

export const CREATE_PROJECT_GROUPS = 'updateProjectGroup';

export const updateProjectGroup = async (
    id: Uuid,
    data: UpdateProjectGroup,
): Promise<ProjectGroup> => axios.put(
        `/api/project-groups/${id}`,
        data,
    )
    .then(x => x.data);

export interface UpdateProjectGroup {
    name: string;
    description?: string;
}

