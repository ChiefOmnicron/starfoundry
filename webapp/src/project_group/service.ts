import type { CharacterId, Uuid } from '@/sdk/utils';
import axios, { type AxiosResponse } from 'axios';

const PROJECT_GROUP_PATH = '/api/v1/project-groups';

export class ProjectGroupService {
    public static cache: { [groupId: ProjectGroupId]: ProjectGroup } = {};

    public static async list(
        filter: IProjectGroupFilter,
    ): Promise<ProjectGroup[]> {
        return await axios
            .get<ProjectGroupId[]>(PROJECT_GROUP_PATH, {
                params: filter,
            })
            .then((x) =>
                Promise.allSettled(
                    x.data.map((y) => {
                        if (!this.cache) {
                            this.cache = {};
                        }

                        this.cache[y] = new ProjectGroup(y);
                        return this.cache[y].load();
                    }),
                ),
            )
            .then((x) =>
                x
                    .filter((x) => x.status === 'fulfilled')
                    // TS does not understand that it has a value as it is
                    // definitly a successful promise
                    .map((x: any) => x.value),
            );
    }

    public static async fetch(groupId: ProjectGroupId): Promise<ProjectGroup> {
        if (this.cache[groupId] && this.cache[groupId].is_deep_loaded) {
            return this.cache[groupId].load();
        }

        this.cache[groupId] = new ProjectGroup(groupId);
        return this.cache[groupId]
            .load()
            .then((_) => this.cache[groupId].deepInfo())
            .then((_) => this.cache[groupId].load());
    }

    public static async fetchInvite(
        groupId: ProjectGroupId,
    ): Promise<ProjectGroup> {
        this.cache[groupId] = new ProjectGroup(groupId);
        return this.cache[groupId].load();
    }

    public static async create(
        info: ICreateProjectGroup,
    ): Promise<ProjectGroupId> {
        return axios.post(PROJECT_GROUP_PATH, info).then((x) => x.data);
    }

    public static async acceptInvite(groupId: Uuid): Promise<ProjectGroupId> {
        return axios.put(`${PROJECT_GROUP_PATH}/${groupId}/members/invite`);
    }
}

export class ProjectGroupMemberService {
    public static async members(groupId: Uuid): Promise<IProjectGroupMember[]> {
        return axios
            .get(`${PROJECT_GROUP_PATH}/${groupId}/members`)
            .then((x) => x.data);
    }

    public static async accept(
        groupId: Uuid,
        characterId: CharacterId,
    ): Promise<void> {
        return axios
            .put(
                `${PROJECT_GROUP_PATH}/${groupId}/members/${characterId}/accept`,
            )
            .then((_) => {
                delete ProjectGroupService.cache[groupId];
            });
    }

    public static async update(
        groupId: Uuid,
        characterId: CharacterId,
        permissions: IPermissionUpdate,
    ): Promise<void> {
        return axios.put(
            `${PROJECT_GROUP_PATH}/${groupId}/members/${characterId}`,
            permissions,
        );
    }

    public static async remove(
        groupId: Uuid,
        characterId: CharacterId,
    ): Promise<void> {
        return axios.delete(
            `${PROJECT_GROUP_PATH}/${groupId}/members/${characterId}`,
        );
    }
}

export class ProjectGroup {
    private loader: Promise<any> | null;

    private error: string | null = null;
    private deepLoaded: boolean = false;

    private _info: IProjectGroup = <any>null;
    private _members: IProjectGroupMember[] = [];

    public constructor(private _projectGroupId: ProjectGroupId) {
        this.loader = axios
            .get(`${PROJECT_GROUP_PATH}/${this._projectGroupId}`)
            .then((info) => {
                this._info = (<AxiosResponse<IProjectGroup>>info).data;
            })
            .then((_) => (this.loader = null));
    }

    public deepInfo() {
        if (this.deepLoaded) {
            return this;
        }

        this.loader = Promise.all(
            [`${PROJECT_GROUP_PATH}/${this._projectGroupId}/members`].map(
                (endpoint) => axios.get(endpoint),
            ),
        )
            .then(
                axios.spread(
                    (
                        // fun with any, because everything has to be the same -.-
                        members: any,
                    ) => {
                        this._members = (<AxiosResponse<IProjectGroupMember[]>>(
                            members
                        )).data;
                    },
                ),
            )
            .then((_) => (this.loader = null))
            .then((_) => (this.deepLoaded = true));
        return this;
    }

    public async load(): Promise<ProjectGroup> {
        if (this.loader) {
            await this.loader;
            this.loader = null;
        }

        return this;
    }

    get is_deep_loaded(): boolean {
        return this.deepLoaded;
    }

    get id(): ProjectGroupId {
        return this._projectGroupId;
    }

    get name(): string {
        return this._info.name;
    }

    set name(name: string) {
        this._info.name = name;
    }

    get description(): string {
        return this._info.description;
    }

    set description(description: string) {
        this._info.description = description;
    }

    get members(): IProjectGroupMember[] {
        return this._members;
    }

    get owner_name(): string {
        return this._info.owner_name;
    }

    get isGroupOwner(): boolean {
        return this._info.is_owner;
    }

    public async update(): Promise<any> {
        return axios.put(`${PROJECT_GROUP_PATH}/${this.id}`, this._info);
    }

    public async remove(): Promise<void> {
        return await axios.delete(`${PROJECT_GROUP_PATH}/${this.id}`);
    }

    public async fetchDefault(): Promise<IProjectGroupDefault> {
        return await axios
            .get<any>(`${PROJECT_GROUP_PATH}/${this.id}/default`)
            .then((x) => x.data);
    }

    public async updateDefault(
        defaults: IProjectGroupDefault,
    ): Promise<IProjectGroupDefault> {
        return await axios.put(
            `${PROJECT_GROUP_PATH}/${this.id}/default`,
            defaults,
        );
    }
}

export type ProjectGroupId = Uuid;

export interface IProjectGroupFilter {
    structures?: string;
    projects?: string;
}

export interface IProjectGroup {
    id?: string;
    name: string;
    description: string;
    owner: number;
    owner_name: string;
    is_owner: boolean;
}

export interface ICreateProjectGroup {
    name: string;
    description: string;
}

export interface IProjectGroupMember {
    character_name: string;
    character_id: number;

    accepted: boolean;
    projects: string;
    project_group: string;
    structures: string;
    is_owner: boolean;
}

export interface IPermissionUpdate {
    projects: string;
    project_group: string;
    structures: string;
}

export interface IProjectGroupDefault {
    markets: Uuid[];
    blacklist: number[];
}
