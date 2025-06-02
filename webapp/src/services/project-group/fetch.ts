import axios from "axios";
import type { Uuid } from "../utils";

export const FETCH_PROJECT_GROUPS = 'fetchProjectGroups';

export const fetchProjectGroup = async (
    projectGroupUuid: Uuid,
): Promise<ProjectGroup[]> => axios.get(
        `/api/project-groups/${projectGroupUuid}`
    )
    .then(x => x.data);

export type ProjectGroup = {
    id: Uuid,
    name: string;
    members: number;
    projects: number;

    description?: string;
}
