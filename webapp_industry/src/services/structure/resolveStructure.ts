import { axiosClient } from "@/services/client";
import type { Item } from "@/services/item/model";
import type { TypeId } from "../utils";

export const RESOLVE_STRUCTURE = "resolveStructure";

export const resolveStructure = async (
    structureId: number,
): Promise<ResolveStructureResponse> =>
    (await axiosClient())
        .get(`/api/eve-gateway/structures/${structureId}`)
        .then((x) => x.data);

export type ResolveStructureResponse = {
    name: string;
    position: {
        x: number;
        y: number;
        z: number;
    };
    structure_id: number;
    system: {
        constellation_id: number;
        constellation_name: string;
        region_id: number;
        region_name: string;
        security: number;
        system_id: number;
        system_name: string;
        security_str: 'HIGHSEC' | 'LOWSEC' | 'NULLSEC';
    };
    item: Item;
    rigs: StructureRig[];
    services: StructureService;
};

export type StructureRig = {
    item:     Item,
    excludes: TypeId[],
}

export type StructureService = {
    services: Item[],
    slots:    number,
}
