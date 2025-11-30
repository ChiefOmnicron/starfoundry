import { axiosClient } from "@/services/client";
import { useQuery } from "@tanstack/react-query";
import type { StructureGroup } from "@/services/structure-group/list";
import type { Uuid } from "@/services/utils";

export const FETCH_STRUCTURE_GROUP = 'fetchStructureGroup';

export const fetchStructureGroup = async (
    structureGroupId: Uuid,
): Promise<StructureGroup> => (await axiosClient())
    .get(
        `/api/structure-groups/${structureGroupId}`
    )
    .then(x => x.data);

// For general use
export const useFetchStructureGroup = (
    id:      Uuid,
) => {
    return useQuery(fetchStructureGroupQuery(id));
}

// For pre-fetching
export const fetchStructureGroupQuery = (
    id:      Uuid,
) => ({
    queryKey: [FETCH_STRUCTURE_GROUP, id],
    queryFn: async () => fetchStructureGroup(id),
    // ms * s * m
    staleTime: 1000 * 60 * 5,
});
