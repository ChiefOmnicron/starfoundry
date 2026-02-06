import { axiosClient } from "@/services/client";
import type { Item } from "@/services/item/model";
import type { StructureRig, StructureService, System } from "./list";

export const RESOLVE_STRUCTURE = "resolveStructure";

export const resolveStructure = async (
    structureId: number,
): Promise<ResolveStructureResponse> =>
    (await axiosClient())
        .get(`/api/eve/structures/${structureId}`)
        .then((x) => x.data);

export type ResolveStructureResponse = {
    name: string;
    position: {
        x: number;
        y: number;
        z: number;
    };
    structure_id: number;
    system: System;
    item: Item;
    installable_rigs: StructureRig[];
    installable_services: StructureService;
};
