import { axiosClient, type AbortSignal } from "@/services/client";
import { useQuery } from "@tanstack/react-query";
import type { ProjectGroupMember } from "@/services/project-group/listMembers";
import type { Uuid } from "@/services/utils";
import type { GenericAbortSignal } from "axios";

export const FETCH_PROJECT_GROUP_MEMBERS_SELF = 'fetchProjectGroupMembersSelf';

export const fetchProjectGroupMemberSelf = async (
    projectGroupUuid: Uuid,
    signal?:          GenericAbortSignal,
): Promise<ProjectGroupMember> => (await axiosClient())
    .get(
        `/api/project-groups/${projectGroupUuid}/members/self`,
        {
            signal,
        }
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
    queryFn: async ({
        signal
    }: AbortSignal) => fetchProjectGroupMemberSelf(projectGroupUuid, signal),
    // ms * s * m
    staleTime: 1000 * 60 * 5,
});
