import { axiosClient } from "../axiosClient";
import type { Uuid } from "../utils";

export const updateIndustryHubs = async (
    projectGroupUuid: Uuid,
    industryHubUuids: Uuid[],
): Promise<void> => (await axiosClient())
    .put(
        `/api/project-groups/${projectGroupUuid}/industry-hubs`,
        industryHubUuids,
    );
