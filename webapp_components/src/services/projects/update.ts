import { axiosClient } from "@internal/services/client";
import type { Uuid } from "@internal/services/utils";
import type { ProjectStatus } from "./list";

export const updateProject = async (
    projectId:  Uuid,
    data:       UpdateProjectRequest,
): Promise<void> => (await axiosClient())
    .put(
        `/api/projects/${projectId}`,
        data,
    )
    .then(x => {
        return x.data;
    });

export type UpdateProjectRequest = {
    project_group_id:   Uuid;
    orderer:            string;
    name:               string;
    status:             ProjectStatus;
    tags:               Uuid[];

    sell_price?:        number;
    note?:              string;
}
