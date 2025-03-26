import type { IParsedRow } from '@/services/item';
import type { Uuid } from '@/utils';
import axios from 'axios';

const PROJECT_PATH: string = '/api/v1/projects';

export class Service {
    public static async fetch_jobs(
        projectId: ProjectId,
    ): Promise<IJobGroup[]> {
        return (await axios.get(`${PROJECT_PATH}/${projectId}/jobs`)).data;
    }

    public static async fetch_misc(
        projectId: ProjectId,
    ): Promise<IMisc[]> {
        return (await axios.get(`${PROJECT_PATH}/${projectId}/misc`)).data;
    }

    public static async fetch_market_recommendation_gas(
        projectId: ProjectId,
    ): Promise<IMarketPriceEntry[]> {
        return (await axios.get(`${PROJECT_PATH}/${projectId}/market/prices/gas`)).data;
    }

    public static async fetch_market_recommendation_general(
        projectId: ProjectId,
    ): Promise<IMarketPriceEntry[]> {
        return (await axios.get(`${PROJECT_PATH}/${projectId}/market/prices`)).data;
    }

    public static async fetch_market_recommendation_minerals(
        projectId: ProjectId,
    ): Promise<IMarketPriceEntry[]> {
        return (await axios.get(`${PROJECT_PATH}/${projectId}/market/prices/minerals`)).data;
    }

    public static async add_misc(
        projectId: ProjectId,
        misc: IMisc,
    ): Promise<void> {
        return await axios.post(`${PROJECT_PATH}/${projectId}/misc`, misc);
    }

    public static async remove_misc(
        projectId: ProjectId,
        misc_uuid: string,
    ): Promise<void> {
        return await axios.delete(`${PROJECT_PATH}/${projectId}/misc/${misc_uuid}`);
    }

    public static async update_market_gas(
        projectId: ProjectId,
        market: IUpdateMarketEntry[]
    ): Promise<void> {
        return await axios.put(`${PROJECT_PATH}/${projectId}/market/gas`, market);
    }

    public static async update_market_general(
        projectId: ProjectId,
        market: IUpdateMarketEntry[]
    ): Promise<void> {
        return await axios.put(`${PROJECT_PATH}/${projectId}/market/bulk`, market);
    }

    public static async update_market_minerals(
        projectId: ProjectId,
        market: IUpdateMarketEntry[]
    ): Promise<void> {
        return await axios.put(`${PROJECT_PATH}/${projectId}/market/minerals`, market);
    }

    public static async fetch_jobs_by_status(
        projectId: ProjectId,
    ): Promise<IJob[]> {
        return (await axios.get<IJobGroup[]>(`${PROJECT_PATH}/${projectId}/jobs/startable`))
            .data
            .flatMap(x => x.entries);
    }

    public static async cost_estimate(
        data: ICostEstimateRequest,
    ): Promise<ICostEstimateResponse> {
        return (await axios.post<ICostEstimateResponse>(`${PROJECT_PATH}/cost-estimate`, data)).data;
    }
}

export type ProjectId = Uuid;
export type JobId = Uuid;

export interface IProjectInfo {
    orderer?:    string;
    notes?:      string;

    sell_price?: number;
    excess:      number;
    jobs:        number;
    market:      number;
    misc:        number;
}

export enum ProjectStatus {
    PREPARING,
    IN_PROGRESS,
    PAUSED,
    ABORTED,
    DONE
}

export interface IProject {
    id: Uuid;
    name: string;
    owner: number;
    status: ProjectStatus | 'IN_PROGRESS' | 'PREPARING' | 'ABORTED' | 'PAUSED' | 'DONE';
    products: IProduct[];
}

export interface ICreateProject {
    name: string;
    products: IProduct[];
    additional_products: IProduct[];
    stocks: IParsedRow[];

    orderer?: string;
    notes?: string;
    sell_price?: number;

    structures: {
        manufacturing: string,
        reaction: string,
    };

    google_sheet?: string;
}

export interface IProduct {
    name: string;
    count: number;
    material_efficiency: number;
    type_id: number;
}

export interface IStock {
    quantity: number;
    type_id: number;
}

export interface IJobGroup {
    header: string;
    entries: IJob[];
}

export interface IJob {
    id: Uuid;
    type_id: number;
    runs: number;
    status: string;
    cost?: number;
    item_name: String,
    structure_uuid: Uuid;
}

export interface IMarket {
    header: string;
    entries: IMarketEntry[];
}

export interface IMarketEntry {
    type_id: number;
    item_name: string;
    quantity: number;
    cost?: number;
    source?: number;
}

export interface IMisc {
    id?: Uuid;
    item: string;
    cost: number;
    quantity?: number;
    description?: string;
}

export interface IUpdateMarketEntry {
    type_id: number;
    quantity: number;
    cost?: number;
    system_id?: number;
}

export interface IMarketPriceEntry {
    source: string;
    type_id: number;
    item_name: string;
    quantity: number;
    remaining: number;
    volume: number;
    price: number;
    structure_id: Uuid,

    price_total?: number;
}

export interface IExcess {
    item_name: string;
    type_id: number;
    quantity: number;
    cost: number;
}

export interface IItem {
    name:          String,
    volume:        number,
    base_price?:   number,

    category_id:   number,
    group_id:      number,
    type_id:       number,
    meta_group_id: number,
}

export interface ICostEstimateRequest {
    products:        IProduct[];
    structure_group: Uuid;
    stocks:          IParsedRow[];
}

export interface ICostEstimateResponse {
    manufacturing_cost_total: number,
    excess_cost_total: number,
    market_cost_total: number,
    total_cost: number,
    market: { type_id: number, quantity: number, price: number }[],

    excess_entries: IExcessCostEntry[];
}

export interface IExcessCostEntry {
    type_id: number;
    quantity: number;
    cost: number;
}

export interface IUpdateJob {
    cost?: number,
    id: string,
    job_id: string,
    status: string,
}
