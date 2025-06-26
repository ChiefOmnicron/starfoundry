import axios from "axios";
import type { Uuid } from "../utils";
import { useQuery } from "@tanstack/react-query";

export const CAN_WRITE_PROJECT_GROUP = 'canWriteProjectGroup';

export const canWriteProjectGroup = async (
    projectGroupUuid: Uuid,
): Promise<boolean> => axios.get(
        `/api/project-groups/${projectGroupUuid}/can-write`
    )
    .then(_ => true)
    .catch(_ => false);

// For general use
export const useCanWriteProjectGroup = (
    projectGroupUuid: Uuid,
) => {
    return useQuery(canWriteProjectGroupQuery(projectGroupUuid));
}

// For pre-fetching
export const canWriteProjectGroupQuery = (
    projectGroupUuid: Uuid,
) => ({
    queryKey: [CAN_WRITE_PROJECT_GROUP, projectGroupUuid],
    queryFn: async () => canWriteProjectGroup(projectGroupUuid),
    initialData: false,
});
