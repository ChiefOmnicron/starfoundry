import axios from "axios";
import type { Uuid } from "../utils";

export const LIST_PROJECT_GROUPS = 'listProjectGroups';

export const listProjectGroups = async (
    filter: ProjectGroupFilter,
): Promise<Uuid[]> => axios.get(
        '/api/project-groups',
        {
            params: filter,
        }
    )
    .then(x => x.data);

export type ProjectGroupFilter = {
    name?: string;
}
