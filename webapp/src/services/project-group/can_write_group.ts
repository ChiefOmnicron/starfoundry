import { axiosClient } from "@/services/client";
import { useQuery } from "@tanstack/react-query";
import type { Uuid } from "@/services/utils";

export const CAN_WRITE_PROJECT_GROUP = 'canWriteProjectGroup';

export const canWriteProjectGroup = async (
    projectGroupUuid: Uuid,
): Promise<boolean> => (await axiosClient())
    .get(
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
