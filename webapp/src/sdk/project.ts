import type { CategoryId, GroupId, JobUuid, ProjectUuid, TypeId, Uuid } from "@/sdk/utils";
import axios, { type AxiosResponse } from "axios";
import type { IParsedRow } from "@/services/item";

const PROJECT_PATH = '/api/v1/projects'
const PROJECT_PATH_JOB_ASSIGNMENTS = `${PROJECT_PATH}/job-assignments`;

export class ProjectService {
    public static cache: { [projectId: ProjectUuid]: Project } = {};

    public static async list(
        filter: IProjectFilter,
    ): Promise<Project[]> {
        return await axios
            .get<ProjectUuid[]>(PROJECT_PATH, {
                params: filter,
            })
            .then(x => {
                if (x.status === 204) {
                    return [];
                } else {
                    return x.data;
                }
            })
            .then(x => Promise.allSettled(
                x.map(y => {
                    if (!this.cache) {
                        this.cache = {};
                    }

                    this.cache[y] = new Project(y);
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
        projectId: ProjectUuid,
    ): Promise<Project> {
        if (this.cache[projectId]) {
            return this.cache[projectId].load();
        }

        this.cache[projectId] = new Project(projectId);
        return this.cache[projectId]
            .load()
            .then(x => x.deep_info())
            .then(x => x.load());
    }

    public static async create_job_assignment(
        job_ids: Uuid[],
    ): Promise<Uuid> {
        return axios
            .post(`${PROJECT_PATH_JOB_ASSIGNMENTS}`, {
                job_ids,
            })
            .then(x => x.data);
    }

    public static async fetch_job_assignment(
        assignment_id: Uuid,
    ): Promise<IJobAssignmentGroup[]> {
        return axios
            .get(`${PROJECT_PATH_JOB_ASSIGNMENTS}/${assignment_id}`)
            .then(x => x.data);
    }

    public static async update_job_assignment(
        assignment_id: Uuid,
        job_id:        Uuid,
    ): Promise<IJobAssignment> {
        return axios
            .put(`${PROJECT_PATH_JOB_ASSIGNMENTS}/${assignment_id}/${job_id}/state`)
            .then(x => x.data);
    }

    public static async check_resources(
        check: ICheckResources
    ): Promise<{ type_id: TypeId, quantity: number }[]> {
        return axios
            .post(`${PROJECT_PATH}/check`, check)
            .then(x => x.data);
    }

    public static async create(
        project: ICreateProject
    ): Promise<ProjectUuid> {
        return axios
            .post(`${PROJECT_PATH}`, project)
            .then(x => x.data);
    }

    public static async lastMarketFetch(
        structureId: Uuid
    ): Promise<string> {
        return axios
            .get(`${PROJECT_PATH}/market/${structureId}/last-fetch`)
            .then(x => x.data);
    }
}

export class Project {
    private PROJECT_PATH: string = '/api/v1/projects';

    private loader: Promise<any> | null;
    private deep_loaded: boolean = false;

    private error: string | null = null;

    private _name: string          = 'Unknown project';

    private _info: IProject         = <any> null;
    private _products: IProduct[]   = [];
    private _jobs: IJobGroup[]      = [];
    private _market: IMarketGroup[] = [];
    private _excess: IExcessGroup[] = [];
    private _misc: IMisc[]          = [];
    private _stock: IStockGroup[]   = [];

    private _canWrite: boolean = false;
    private _isOwner: boolean = false;

    public constructor(
        private _projectId: ProjectUuid,
    ) {
        this.loader = axios
            .get(`${this.PROJECT_PATH}/${this._projectId}`)
            .then(info => {
                this._info     = (<AxiosResponse<IProject>> info).data;
                this._name     = this._info.name;
                this._products = this._info.products;
            })
            .then(_ => this.loader = null);
    }

    public deep_info() {
        if (this.deep_loaded) {
            return this;
        }

        this.loader = Promise
            .all(
                [
                    `${this.PROJECT_PATH}/${this._projectId}/jobs`,
                    `${this.PROJECT_PATH}/${this._projectId}/market`,
                    `${this.PROJECT_PATH}/${this._projectId}/misc`,
                ]
                .map(endpoint => axios.get(endpoint))
            )
            .then(axios.spread((
                    // fun with any, because everything has to be the same -.-
                    jobs:   any,
                    market: any,
                    misc:   any,
                ) => {
                    this._jobs   = (<AxiosResponse<IJobGroup[]>> jobs).data;
                    this._market = (<AxiosResponse<IMarketGroup[]>> market).data;
                    this._misc   = (<AxiosResponse<IMisc[]>> misc).data;
                })
            )
            .then(_ => this.loader = null)
            .then(_ => this.deep_loaded = true);
        return this;
    }

    public async fetchPermissionIsOwner(): Promise<void> {
        await axios
            .get(`${PROJECT_PATH}/${this._projectId}/permissions/is-owner`)
            .then(_ => this._isOwner = true)
            .catch(_ => this._isOwner = false);
    }

    public async fetchPermissionCanWrite(): Promise<void> {
        await axios
            .get(`${PROJECT_PATH}/${this._projectId}/permissions/can-write`)
            .then(_ => this._canWrite = true)
            .catch(_ => this._canWrite = false);
    }

    public async activeJobs(): Promise<IActiveJob[]> {
        return axios
            .get(`${PROJECT_PATH}/${this._projectId}/jobs/active`)
            .then(x => x.data);
    }

    public async fetchJobs(filter: IFetchJobFilter): Promise<IJob[]> {
        return axios
            .get(`${PROJECT_PATH}/${this._projectId}/jobs`, {
                params: filter,
            })
            .then(x => x.data);
    }

    public async fetchJobsGrouped(filter: IFetchJobFilter): Promise<IJobGroup[]> {
        return axios
            .get(`${PROJECT_PATH}/${this._projectId}/jobs`, {
                params: {
                    grouped: true,
                    ...filter,
                },
            })
            .then(x => x.data);
    }

    public async deleteJob(jobId: JobUuid): Promise<void> {
        return axios
            .delete(`${PROJECT_PATH}/${this._projectId}/jobs/${jobId}`)
            .then(x => x.data);
    }

    public async updateJob(
        jobId: JobUuid,
        job:   IJob,
    ): Promise<void> {
        return axios
            .put(`${PROJECT_PATH}/${this._projectId}/jobs/${jobId}`, job)
            .then(x => x.data);
    }

    public addMarket(
        entry: IMarket,
    ): Promise<void> {
        return axios
            .post(`${PROJECT_PATH}/${this._projectId}/market`, entry);
    }

    public fetchMarket(): Promise<IMarketGroup[]> {
        return axios
            .get(`${PROJECT_PATH}/${this._projectId}/market`)
            .then(x => x.data);
    }

    public updateMarket(
        marketUuid: ProjectMarketUuid,
        update:     IMarket,
    ): Promise<void> {
        return axios
            .put(
                `${PROJECT_PATH}/${this._projectId}/market/${marketUuid}`,
                update
            );
    }

    public deleteMarket(
        marketUuid: ProjectMarketUuid,
    ): Promise<IMarketGroup[]> {
        return axios
            .delete(`${PROJECT_PATH}/${this._projectId}/market/${marketUuid}`)
            .then(x => x.data);
    }

    public async fetchCompressedOre(
        efficiency: REPROCESSING_EFFICIENCY,
    ): Promise<ICompressedOres> {
        return axios
            .get(`${PROJECT_PATH}/${this._projectId}/compressed`, {
                params: {
                    reprocessing: efficiency,
                },
            })
            .then(x => x.data);
    }

    public async fetchExcess(): Promise<IExcessGroup[]> {
        return axios
            .get(`${PROJECT_PATH}/${this._projectId}/excess`)
            .then(x => {
                this._excess = x.data;
                return x.data;
            });
    }

    public async updateExcessPrices(
        selectedAppraisal: 'INTERNAL' | 'JANICE'
    ): Promise<IExcessGroup[]> {
        return axios.put(
            `${PROJECT_PATH}/${this._projectId}/excess/prices`,
            {
                appraisal: selectedAppraisal
            }
        )
        .then(_ => this.fetchExcess());
    }

    public async fetchStock(): Promise<IStockGroup[]> {
        return axios
            .get(`${PROJECT_PATH}/${this._projectId}/stocks`)
            .then(x => {
                this._stock = x.data;
                return x.data;
            });
    }

    public async updateStockPrices(
        selectedAppraisal: 'INTERNAL' | 'JANICE'
    ): Promise<IStockGroup[]> {
        return axios.put(
            `${PROJECT_PATH}/${this._projectId}/stocks/prices`,
            {
                appraisal: selectedAppraisal
            }
        )
        .then(_ => this.fetchStock());
    }

    public async updateCompressedOre(
        // TODO: replace IParsedRow
        ores: IParsedRow[],
    ): Promise<void> {
        let compressed = ores
            .map(x => {
                return {
                    type_id: x.type_id,
                    quantity: x.quantity,
                }
            });

        return axios
            .put(`${PROJECT_PATH}/${this._projectId}/market/minerals`, compressed);
    }

    public async load(): Promise<Project> {
        if (this.loader) {
            await this.loader;
            this.loader = null;
        }

        return this;
    }

    public async saveSettings(): Promise<any> {
        return axios.put(
            `${this.PROJECT_PATH}/${this.id}`,
            {
                name: this._info.name,
                status: this._info.status,
                orderer: this._info.orderer,
                sell_price: this._info.finance.sell_price,
                notes: this._info.notes,
                project_group_id: this._info.project_group_id,
            },
        );
    }

    public async remove(): Promise<void> {
        return await axios.delete(`${this.PROJECT_PATH}/${this.id}`)
    }

    get id(): ProjectUuid {
        return this._projectId;
    }

    get name(): string {
        return this._name;
    }

    set name(name: string) {
        this._name = name;
        this._info.name = name;
    }

    get notes(): string {
        return this._info.notes || '';
    }

    get status(): string {
        return this._info.status;
    }

    get info(): IProject {
        console.error('called Project.info')
        return this._info;
    }

    get finance(): IFinance {
        return this._info.finance;
    }

    get orderer(): string {
        return this._info.orderer;
    }

    get price(): number {
        return this._info.finance.sell_price || 0;
    }

    get cost(): number {
        return this.finance.jobs + this.finance.market + this.finance.misc - this.finance.excess;
    }

    get products(): IProduct[] {
        return this._info.products;
    }

    get canWrite(): boolean {
        return this._canWrite;
    }

    get isOwner(): boolean {
        return this._isOwner;
    }

    get minerals(): IMarket[] {
        let entries = this._market.find(x => x.header === 'MINERALS');
        if (entries) {
            return entries
                .entries
                .filter((x: any) => x.type_id !== 11399)
        } else {
            return [];
        }
    }
}

type ProjectStatus = 'PREPARING' | 'IN_PROGRESS' | 'PAUSED' | 'ABORTED' | 'DONE' | 'UNKNOWN';
type JobStatus     = 'WAITING_FOR_MATERIALS' | 'BUILDING' | 'DONE';

export type ProjectMarketUuid = Uuid;

export interface IProjectFilter {
    name?:   string;
    status?: 'PREPARING' | 'IN_PROGRESS' | 'PAUSED' | 'DONE';
}

export interface IProject {
    id:       string;
    name:     string;
    status:   ProjectStatus;
    products: IProduct[];

    sell_price?: number;
    notes?:      string;
    orderer:     string;
    finance:     IFinance;

    project_group_id:   Uuid;
    structure_group_id: Uuid;
}

export interface IFinance {
    excess:      number;
    jobs:        number;
    market:      number;
    misc:        number;
    sell_price:  number;
}

export interface IMisc {
    item_name: string;
    cost:      number;

    quantity?: number;
    type_id?:  string;
}

export interface IStockGroup {
    header:  string;
    entries: IStock[];
}

export interface IStock {
    item_name: string;
    quantity:  number;
    type_id:   TypeId;

    cost?:     number;
}

export interface IExcessGroup {
    header:  string;
    entries: IExcess[];
}

export interface IExcess {
    item_name: string;
    quantity:  number;
    type_id:   TypeId;

    cost?:     number;
}

export interface IProduct {
    count:               number;
    material_efficiency: number;
    type_id:             TypeId;
}

export interface IJobGroup {
    header:  string;
    entries: IJob[];
}

export interface IJob {
    id:        Uuid;
    status:    JobStatus;
    runs:      number;
    type_id:   TypeId,
    item_name: string;

    cost?:     number;
    job_id?:   JobUuid;
}

export interface IMarketGroup {
    header:  string;
    entries: IMarket[];
}

export interface IMarket {
    id:          ProjectMarketUuid;
    caregory_id: CategoryId;
    group_id:    GroupId;
    item_name:   string;
    quantity:    number;
    type_id:     TypeId;

    cost?:   number;
    source?: string;
}

// TODO: move to ItemService
export interface IItem {
    base_price:  number;
    category_id: CategoryId;
    group_id:    GroupId;
    name:        string;
    type_id:     TypeId;
    volume:      number;

    meta_group_id?: GroupId;
}

export interface IJobAssignmentGroup {
    header:  string;
    entries: IJobAssignment[];
}

export interface IJobAssignment {
    job_id:         Uuid;
    type_id:        TypeId;
    structure_name: string;
    project_name:   string;
    item_name:      string;
    runs:           number;
    started:        boolean;
    category_id:    number;
    group_id:       number;
    meta_group_id:  number;
}

export interface ICheckResources {
    job_ids:   Uuid[];
    resources: IItemQuantity[];
}

export interface IItemQuantity {
    type_id:  TypeId;
    quantity: number;
}

export interface IJobToStart {
    type_id:      TypeId;
    runs:         number;
    structure_id: Uuid;
}

export interface IActiveJob {
    id:             Uuid;
    type_id:        TypeId;
    runs:           number;
    status:         'DONE' | 'BUILDING' | 'WAITING_FOR_MATERIALS';
    structure_id:   number;
    cost?:          number;
    job_id?:        number;
    delivered:      boolean;
    end_date:       string;
    item:           IItem;
    activity:       'MANUFACTURING' | 'TIME_EFFICIENCY_RESEARCH' | 'MATERIAL_EFFICIENCY_RESEARCH' | 'COPYING' | 'INVENTION' | 'REACTIONS';
    structure_name: string;
    remaining?:     number;
}

export type REPROCESSING_EFFICIENCY = 'NsTataraT1' | 'NsTataraT2';

export interface ICompressedOres {
    reprocessing_efficiency: number,
    goal:                    { [key: TypeId]: number };
    compressed:              IICompressedMineralsEntry[];
    reprocessed:             IICompressedMineralsEntry[];
    overage:                 IICompressedMineralsEntry[];
}

export interface IICompressedMineralsEntry {
    type_id: TypeId;
    amount:  number;
    price:   number;
}

export interface ICreateProject {
    name: string;
    products: IProduct[];
    additional_products: IProduct[];
    stocks: IParsedRow[];

    orderer?: string;
    notes?: string;
    sell_price?: number;

    structure_group_id: string;

    markets: Uuid[];
    blacklist: number[];
}

export interface IFetchJobFilter {
    type_id?: TypeId;
}
