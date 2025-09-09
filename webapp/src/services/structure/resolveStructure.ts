import { axiosClient } from "@/services/client";
import type { Item } from "@/services/item/model";

export const RESOLVE_STRUCTURE = "resolveStructure";

export const resolveStructure = async (
    structureId: number,
): Promise<ResolveStructureResponse> =>
    (await axiosClient())
        .get(`/api/structures/resolve/${structureId}`)
        .then((x) => x.data);

export type ResolveStructureResponse = {
    name: string;
    position: {
        x: number;
        y: number;
        z: number;
    };
    structure_id: number;
    system_id: {
        constellation_id: number;
        constellation_name: string;
        region_id: number;
        region_name: string;
        security: number;
        system_id: number;
        system_name: string;
    };
    type_id: Item;
};
