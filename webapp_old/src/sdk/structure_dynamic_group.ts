import { Structure, StructureService } from './structure';
import type { Uuid } from '@/sdk/utils';
import axios from 'axios';

const STRUCTURE_PATH: string = '/api/v1/structures/groups/dynamic';

export type StructureDynamicGroupId = Uuid;

export class StructureDynamicGroupService {
    public static cache: {
        [structure_group_id: StructureDynamicGroupId]: StructureDynamicGroup;
    } = {};

    public static async all(): Promise<StructureDynamicGroup[]> {
        return await axios
            .get<StructureDynamicGroupId[]>(STRUCTURE_PATH)
            .then((x) => {
                if (x.status === 204) {
                    return [{ status: 'None' }];
                }

                return Promise.allSettled(
                    x.data.map((y) => {
                        this.cache[y] = new StructureDynamicGroup(y);
                        return this.cache[y].load();
                    }),
                );
            })
            .then((x) =>
                x
                    .filter((x) => x.status === 'fulfilled')
                    // TS does not understand that it has a value, as it is
                    // definitly a successful promise
                    .map((x: any) => x.value),
            );
    }

    public static async by_id(
        structure_group_id: StructureDynamicGroupId,
    ): Promise<StructureDynamicGroup> {
        if (this.cache[structure_group_id]) {
            return this.cache[structure_group_id].load();
        }

        this.cache[structure_group_id] = new StructureDynamicGroup(
            structure_group_id,
        );
        return this.cache[structure_group_id].load();
    }

    public static async create(
        structure_group: IStructureDynamicGroup,
    ): Promise<StructureDynamicGroup> {
        return axios
            .post(`${STRUCTURE_PATH}`, structure_group)
            .then((x) => this.by_id(x.data));
    }

    public static async remove(
        structure_group_id: StructureDynamicGroupId,
    ): Promise<void> {
        await axios.delete(`${STRUCTURE_PATH}/${structure_group_id}`);
        delete this.cache[structure_group_id];
        return;
    }

    public static async resolve_manufacturing(
        filter: string[],
    ): Promise<Structure[]> {
        return axios
            .post<
                Uuid[]
            >(`${STRUCTURE_PATH}/dynamic/resolve/manufacturing`, filter)
            .then((x) => {
                return Promise.all(
                    x.data.map((y) => {
                        return StructureService.fetch(y);
                    }),
                );
            });
    }

    public static async resolve_reaction(
        filter: string[],
    ): Promise<Structure[]> {
        return axios
            .post<Uuid[]>(`${STRUCTURE_PATH}/dynamic/resolve/reaction`, filter)
            .then((x) => {
                return Promise.all(
                    x.data.map((y) => {
                        return StructureService.fetch(y);
                    }),
                );
            });
    }
}

export class StructureDynamicGroup {
    private loader: Promise<any> | null;

    private _info: IStructureDynamicGroup = <any>null;

    public constructor(private _id: StructureDynamicGroupId) {
        this.loader = axios
            .get(`${STRUCTURE_PATH}/${this._id}`)
            .then((x) => (this._info = x.data));
    }

    public async load(): Promise<StructureDynamicGroup> {
        if (this.loader) {
            await this.loader;
            this.loader = null;
        }

        return this;
    }

    get id(): StructureDynamicGroupId {
        return this._id;
    }

    get name(): string {
        return this._info.name;
    }

    get group_ids(): string[] {
        return this._info.group_ids;
    }
}

export interface IStructureDynamicGroup {
    id?: Uuid;
    name: string;

    group_ids: string[];
}
