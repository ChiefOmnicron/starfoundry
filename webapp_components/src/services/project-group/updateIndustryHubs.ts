import { axiosClient } from "@internal/services/client";
import type { Uuid } from "@internal/services/utils";

export const updateIndustryHubs = async (
    projectGroupUuid: Uuid,
    industryHubUuids: Uuid[],
): Promise<void> => (await axiosClient())
    .put(
        `/api/project-groups/${projectGroupUuid}/industry-hubs`,
        industryHubUuids,
    );
