import axios from "axios";
import type { Uuid } from "../utils";
import { useQuery } from "@tanstack/react-query";

export const FETCH_PROJECT_GROUP_MEMBER = 'fetchProjectGroupMember';

export const fetchProjectGroupMember = async (
    projectGroupUuid: Uuid,
): Promise<ProjectGroupMember> => axios.get(
        `/api/project-groups/${projectGroupUuid}/members`
    )
    .then(x => x.data);

export type ProjectGroupMember = {
    character_name: string,
    character_id:   number,

    accepted:       boolean,
    is_owner:       boolean,
    permission:     number,
}

// For general use
export const useFetchProjectGroup = (
    projectGroupUuid: Uuid,
) => {
    return useQuery(fetchProjectGroupMemberQuery(projectGroupUuid));
}

// For pre-fetching
export const fetchProjectGroupMemberQuery = (
    projectGroupUuid: Uuid,
) => ({
    queryKey: [FETCH_PROJECT_GROUP_MEMBER, projectGroupUuid],
    queryFn: async () => fetchProjectGroupMember(projectGroupUuid),
});
