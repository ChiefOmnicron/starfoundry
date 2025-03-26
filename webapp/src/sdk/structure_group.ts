import axios from "axios";
import type { StructureGroupId, StructureId, Uuid } from "@/sdk/utils";
import { Structure, StructureService } from "./structure";

const STRUCTURE_PATH: string = '/api/v1/structures/groups';

export class StructureGroupService {
    public static cache: { [structure_group_id: StructureGroupId]: StructureGroup } = {};

    public static async all(): Promise<StructureGroup[]> {
        return await axios
            .get<StructureGroupId[]>(STRUCTURE_PATH)
            .then(x => {
                if (x.status === 204) {
                    return [];
                }

                return x.data;
            })
            .then(x => Promise.allSettled(
                x
                    .map(y => {
                        this.cache[y] = new StructureGroup(y);
                        return this.cache[y].load();
                    }))
            )
            .then(x => x
                .filter(x => x.status === 'fulfilled')
                // TS does not understand that it has a value as it is
                // definitly a successful promise
                .map((x: any) => x.value)
            );
    }

    public static async fetch(
        structure_group_id: StructureGroupId,
    ): Promise<StructureGroup> {
        if (this.cache[structure_group_id]) {
            return this.cache[structure_group_id].load();
        }

        this.cache[structure_group_id] = new StructureGroup(structure_group_id);
        return this.cache[structure_group_id].load();
    }

    public static async create(
        structure_group: IStructureCreateGroup,
    ): Promise<StructureGroup> {
        return axios
            .post(`${STRUCTURE_PATH}`, structure_group)
            .then(x => this.fetch(x.data));
    }

    public static async remove(
        structure_group_id: StructureGroupId,
    ): Promise<void> {
        await axios.delete(`${STRUCTURE_PATH}/${structure_group_id}`);
        delete this.cache[structure_group_id];
        return;
    }
}

export class StructureGroup {
    private loader: Promise<any> | null;

    private _info: IStructureGroup   = <any> null;
    private _structures: Structure[] = [];

    public constructor(
        private _structure_group_id: StructureGroupId,
    ) {
        this.loader = axios
            .get(`${STRUCTURE_PATH}/${this._structure_group_id}`)
            .then(x => this._info = x.data)
            .then(_ => StructureService.bulkLoad(this._info.structure_ids))
            .then(x => this._structures = x);
    }

    public async load(): Promise<StructureGroup> {
        if (this.loader) {
            await this.loader;
            this.loader = null;
        }

        return this;
    }

    get structure_group_id(): string {
        return this._structure_group_id;
    }

    get structures(): Structure[] {
        return this._structures;
    }

    get name(): string {
        return this._info.name;
    }

    get structure_types(): string[] {
        return [...new Set(this._structures.map(x => x.structureName))];
    }

    get structure_services(): number[] {
        return [...new Set(
            this._structures.flatMap(x => x.services)
        )];
    }

    get structure_systems(): number[] {
        return [...new Set(this._structures.map(x => x.systemId))];
    }
}

export interface IStructureGroup {
    id:            Uuid;
    name:          string;

    structure_ids: StructureId[];
}

export interface IStructureCreateGroup {
    name:          string;

    structure_ids: StructureId[];
}
