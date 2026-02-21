import { axiosClient, type AbortSignal } from "@internal/services/client";
import { useQuery } from "@tanstack/react-query";
import type { Uuid } from "@internal/services/utils";
import type { Category, Group, Item } from "@internal/services/item/model";
import type { GenericAbortSignal } from "axios";

export const LIST_STRUCTURE = 'listStructure';

export const listStructure = async (
    filter:  StructureFilter,
    signal?: GenericAbortSignal,
): Promise<Structure[]> => (await axiosClient())
    .get(
        `/api/structures`,
        {
            params: filter,
            signal,
        }
    )
    .then(x => x.data);

export type Structure = {
    id:                     Uuid;
    structure_id:           number;
    name:                   string;
    system:                 System;
    item:                   Item;
    rigs:                   StructureRig[];
    services:               Item[];
    position:               StructurePosition;
    taxes:                  StructureTax,

    installable_rigs?:      StructureRig[];
    installable_services?:  StructureService;
}

export type StructureTax = { [key: number]: number };

export type StructureService = {
    services: Item[],
    slots:    number,
}

export type StructurePosition = {
    x: number;
    y: number;
    z: number;
};

export type System = {
    region_id:          number;
    region_name:        string;
    constellation_id:   number;
    constellation_name: string;
    system_id:          number;
    system_name:        string;
    security:           string;
    security_str:       'HIGHSEC' | 'LOWSEC' | 'NULLSEC';
}

export type StructureRig = {
    item:       Item;
    excludes:   number[];
    categories: Category[];
    groups:     Group[];

    material?:  number;
    time?:      number;
}

export type StructureFilter = {
    name?:              string;
    system_id?:         number;
    structure_type_id?: number;
    service_id?:        number;
    rig_id?:            number;
    include_npc?:       boolean;
}

// For general use
export const useListStructure = (
    filter: StructureFilter,
) => {
    return useQuery(listStructureQuery(filter));
}

// For pre-fetching
export const listStructureQuery = (
    filter: StructureFilter,
) => ({
    queryKey: [LIST_STRUCTURE, filter],
    queryFn: async ({
        signal
    }: AbortSignal) => listStructure(filter, signal),
    // ms * s * m
    staleTime: 1000 * 60 * 5,
});
