import { axiosClient } from "@/services/client";

export const RESOLVE_STRUCTURE = 'resolveStructure';

export const resolveStructure = async (
    structureId: number,
): Promise<ResolveStructureResponse> => (await axiosClient())
    .get(
        `/api/structures/resolve/${structureId}`,
    )
    .then(x => x.data);

export type ResolveStructureResponse {

}
