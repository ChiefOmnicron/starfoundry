import { axiosClient } from "@/services/client";
import { useQuery } from "@tanstack/react-query";
import type { Uuid } from "@/services/utils";

export const LIST_PROJECT_GROUP_MEMBERS = 'listProjectGroupMembers';

export const listProjectGroupMembers = async (
    projectGroupUuid: Uuid,
): Promise<ProjectGroupMember> => (await axiosClient())
    .get(
        `/api/project-groups/${projectGroupUuid}/members`
    )
    .then(x => x.data);

export type ProjectGroupPermission = 'OWNER' |
    'READ' |
    'WRITE_PROJECT' |
    'WRITE_STRUCTURE' |
    'WRITE_DEFAULT' |
    'WRITE_MEMBER' |
    'WRITE_GROUP';
export type ProjectGroupMember = {
    character_name: string,
    character_id:   number,

    accepted:       boolean,
    is_owner:       boolean,
    permissions:    ProjectGroupPermission[],
}

// For general use
export const useFetchProjectGroup = (
    projectGroupUuid: Uuid,
) => {
    return useQuery(listProjectGroupMembersQuery(projectGroupUuid));
}

// For pre-fetching
export const listProjectGroupMembersQuery = (
    projectGroupUuid: Uuid,
) => ({
    queryKey: [LIST_PROJECT_GROUP_MEMBERS, projectGroupUuid],
    queryFn: async () => listProjectGroupMembers(projectGroupUuid),
});
