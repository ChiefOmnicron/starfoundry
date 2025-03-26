import axios from "axios";
import type { StructureId, TypeId, Uuid } from "@/sdk/utils";

const STRUCTURE_PATH: string = '/api/v1/structures';

export const STRUCTURE_TYPE_ID_NAME = (type_id: TypeId) => {
    switch(type_id) {
        case 35835:
            return 'Athanor'
        case 35836:
            return 'Tatara'

        case 35825:
            return 'Raitaru'
        case 35826:
            return 'Azbel'
        case 35827:
            return 'Sotiyo'

        case 35832:
            return 'Fortizar'
        case 35833:
            return 'Fortizar'
        case 47512:
            return '\'Moreau\' Fortizar'
        case 47513:
            return '\'Draccous\' Fortizar'
        case 47514:
            return '\'Horizon\' Fortizar'
        case 47515:
            return '\'Marginis\' Fortizar'
        case 47516:
            return '\'Prometheus\' Fortizar'
        case 35834:
            return 'Keepstar'
        case 40340:
            return 'Upwell Palatine Keepstar'

        default:
            return 'Unknown'
    }
}

export class StructureService {
    public static cache: { [structure_id: StructureId]: Structure } = {};

    public static async list(
        filter: IStructureFilter,
    ): Promise<Structure[]> {
        return await axios
            .get<StructureId[]>(STRUCTURE_PATH, {
                params: filter,
            })
            .then(x => {
                let response: StructureId[] = [];
                if (x.data) {
                    response = x.data;
                }

                return Promise.allSettled(
                    response
                        .map(y => {
                            if (!this.cache) {
                                this.cache = {};
                            }

                            this.cache[y] = new Structure(y);
                            return this.cache[y].load();
                        })
                    )
            })
            .then(x => x
                .filter(x => x.status === 'fulfilled')
                // TS does not understand that it has a value as it is
                // definitly a successful promise
                .map((x: any) => x.value)
            );
    }

    public static async fetch(
        structure_id: StructureId,
    ): Promise<Structure> {
        if (this.cache[structure_id]) {
            return this.cache[structure_id].load();
        }

        this.cache[structure_id] = new Structure(structure_id);
        return this.cache[structure_id].load();
    }

    public static async create(
        structure: IStructure,
    ): Promise<Structure> {
        return axios
            .post(`${STRUCTURE_PATH}`, structure)
            .then(x => this.fetch(x.data));
    }

    public static async bulkLoad(
        structure_ids: StructureId[],
    ): Promise<Structure[]> {
        return Promise.all(structure_ids.map(x => this.fetch(x)));
    }

    public static async resolve(
        structure_id: number,
    ): Promise<IStructureResolve> {
        return (await axios.get(`${STRUCTURE_PATH}/${structure_id}/resolve`)).data;
    }

    public static async possibleRigs(
        structure_id: TypeId,
    ): Promise<{ type_id: TypeId, name: string }[]> {
        return (await axios.get(`${STRUCTURE_PATH}/${structure_id}/rigs`)).data;
    }

    public static async structureName(
        structure_id: StructureId,
    ): Promise<string> {
        if (this.cache[structure_id]) {
            return (await this.cache[structure_id].load()).name;
        }

        this.cache[structure_id] = new Structure(structure_id);
        return (await this.cache[structure_id].load()).name;
    }
}

export class Structure {
    private loader: Promise<any> | null;

    private _info: IStructure                 = <any> null;
    private _permission: IStructurePermission = <any> null;

    private _canWrite: boolean = false;
    private _isOwner: boolean = false;

    public constructor(
        private _structure_id: StructureId,
    ) {
        if (_structure_id === '00000000-0000-0000-0000-000000000000') {
            this.loader = new Promise((resolve, _) => {
                this._info = {
                    name:              'Unknown Structure',
                    rigs:              [],
                    security:          '',
                    services:          [],
                    structure_id:      0,
                    structure_type_id: 0,
                    system_id:         0,
                    system_name:       'Unknown',
                    project_group_ids: [],

                    // TODO: better way
                    isOwner:           false,
                    canEdit:           false,
                }
                resolve(this);
            });
            return;
        }

        this.loader = axios
            .get(`${STRUCTURE_PATH}/${this._structure_id}`)
            .then(x => this._info = x.data);
    }

    public async load(): Promise<Structure> {
        if (this.loader) {
            await this.loader;
            this.loader = null;
        }

        return this;
    }

    public async fetchPermissionIsOwner(): Promise<void> {
        await axios
            .get(`${STRUCTURE_PATH}/${this._structure_id}/permissions/is-owner`)
            .then(_ => this._isOwner = true)
            .catch(_ => this._isOwner = false);
    }

    public async fetchPermissionCanWrite(): Promise<void> {
        await axios
            .get(`${STRUCTURE_PATH}/${this._structure_id}/permissions/can-write`)
            .then(_ => this._canWrite = true)
            .catch(_ => this._canWrite = false);
    }

    public async patch_rigs(): Promise<void> {
        console.error('patch rigs called');
        await axios.patch(
            `${STRUCTURE_PATH}/${this._structure_id}/rigs`,
            this._info.rigs.filter(x => x !== null),
        );
        delete StructureService.cache[this._structure_id];
    }

    public async patch_services(): Promise<void> {
        console.error('patch services called');
        await axios.patch(
            `${STRUCTURE_PATH}/${this._structure_id}/services`,
            this._info.services
        );
        delete StructureService.cache[this._structure_id];
    }

    public async update() {
        await axios.put(
            `${STRUCTURE_PATH}/${this._structure_id}`,
            this._info,
        );
    }

    public async remove(): Promise<void> {
        await axios.delete(`${STRUCTURE_PATH}/${this._structure_id}`);
        delete StructureService.cache[this._structure_id];
        return;
    }

    get id(): StructureId {
        return this._structure_id;
    }

    get ingameId(): number {
        return this._info.structure_id;
    }

    get name(): string {
        return this._info.name;
    }

    set name(name: string) {
        this._info.name = name;
    }

    get systemId(): number {
        return this._info.system_id;
    }

    get systemName(): string {
        return this._info.system_name;
    }

    get structureName(): string {
        return STRUCTURE_TYPE_ID_NAME(this._info.structure_type_id);
    }

    get structureTypeId(): TypeId {
        return this._info.structure_type_id;
    }

    get projectGroupIds(): Uuid[] {
        return this._info.project_group_ids;
    }

    set projectGroupIds(groupIds: Uuid[]) {
        this._info.project_group_ids = groupIds;
    }

    get rigs(): TypeId[] {
        return this._info.rigs;
    }

    set rigs(rigs: TypeId[]) {
        this._info.rigs = rigs;
    }

    get services(): TypeId[] {
        return this._info.services;
    }

    set services(services: TypeId[]) {
        this._info.services = services;
    }

    get canWrite(): boolean {
        return this._canWrite;
    }

    get isOwner(): boolean {
        return this._isOwner;
    }
}

export interface IStructureFilter {
    name?:       string;
    system_id?:  number;
    type_id?:    TypeId;
    service_id?: TypeId,
}

export interface IStructure {
    name:              string;
    structure_id:      number;

    structure_type_id: TypeId;
    services:          TypeId[];
    rigs:              TypeId[];

    system_id:         number;
    system_name:       string;
    security:          string;
    project_group_ids: Uuid[];

    isOwner:           boolean;
    canEdit:           boolean;
}

export interface IStructureResolve {
    structure_id: number;
    name: string;
    system_id: number;
    type_id: number;
    security: string;
}

export interface IStructurePermission {
    structure_owner: boolean;
}
