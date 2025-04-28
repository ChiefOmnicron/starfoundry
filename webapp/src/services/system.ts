import axios from 'axios';

export class SystemService {
    public static async search_by_id(
        id: number | undefined,
    ): Promise<ISystem[]> {
        if (!id) {
            return (await axios.get(`/api/v1/search/systems`)).data;
        }

        return (await axios.get(`/api/v1/search/systems?system_id=${id}`)).data;
    }

    public static async search_by_name(query: string): Promise<ISystem[]> {
        return (await axios.get(`/api/v1/search/systems?name=${query}`)).data;
    }
}

export interface ISystem {
    region_name: string;
    region_id: number;

    system_name: string;
    system_id: string;
    security: number;
}
