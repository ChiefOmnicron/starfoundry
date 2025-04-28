import axios from 'axios';

const INDUSTRY_JOB_PATH: string = '/api/v1/industry/jobs';
const INDUSTRY_INDEX_PATH: string = '/api/v1/industry/system-index';
const INDUSTRY_WEIGHTED_AVERAGE: string = '/api/v1/market/weighted_average';

export class Service {
    public static async character_jobs(): Promise<IIndustryJob[]> {
        return (await axios.get(`${INDUSTRY_JOB_PATH}`)).data;
    }

    public static async industry_index(
        system_id: number,
    ): Promise<IIndustryIndex[]> {
        return (
            await axios.get(`${INDUSTRY_INDEX_PATH}?system_id=${system_id}`)
        ).data;
    }

    public static async weighted_average(type_ids: {
        [key: number]: number;
    }): Promise<any> {
        return (await axios.post(`${INDUSTRY_WEIGHTED_AVERAGE}`, type_ids))
            .data;
    }
}

export interface IIndustryJob {
    activity_id: string;
    blueprint_type_id: number;
    blueprint_location_id: number;
    status: string;
    start_date: string;
    end_date: string;
    cost: number;
    runs: number;
    job_id: number;
    installer_id: number;

    corporation_id?: number;
    remaining?: number;
}

export interface IIndustryIndex {
    timestamp: number;
    system_id: number;
    reaction: number;
    manufacturing: number;
    invention: number;
    copying: number;
    research_material: number;
    research_time: number;
}
