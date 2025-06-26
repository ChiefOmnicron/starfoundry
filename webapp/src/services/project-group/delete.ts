import axios from "axios";
import type { Uuid } from "../utils";
import type { ProjectGroup } from "./fetch";

export const CREATE_PROJECT_GROUPS = 'updateProjectGroup';

export const deleteProjectGroup = async (
    projectGroupUuid: Uuid,
): Promise<ProjectGroup> => axios.delete(
        `/api/project-groups/${projectGroupUuid}`,
    )
    .then(x => x.data);
