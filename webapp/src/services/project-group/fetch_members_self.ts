import { axiosClient } from "@/services/client";
import { useQuery } from "@tanstack/react-query";
import type { ProjectGroupMember } from "@/services/project-group/list_members";
import type { Uuid } from "@/services/utils";

export const FETCH_PROJECT_GROUP_MEMBERS_SELF = 'fetchProjectGroupMembersSelf';

export const fetchProjectGroupMemberSelf = async (
    projectGroupUuid: Uuid,
): Promise<ProjectGroupMember> => (await axiosClient())
    .get(
        `/api/project-groups/${projectGroupUuid}/members/self`
    )
    .then(x => x.data);

// For general use
export const useFetchProjectGroupMemberSelf = (
    projectGroupUuid: Uuid,
) => {
    return useQuery(fetchProjectGroupMemberSelfQuery(projectGroupUuid));
}

// For pre-fetching
export const fetchProjectGroupMemberSelfQuery = (
    projectGroupUuid: Uuid,
) => ({
    queryKey: [FETCH_PROJECT_GROUP_MEMBERS_SELF, projectGroupUuid],
    queryFn: async () => fetchProjectGroupMemberSelf(projectGroupUuid),
    staleTime: 60 * 10,
});
