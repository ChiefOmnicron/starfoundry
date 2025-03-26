import { type TypeId } from "@/utils";
import axios from "axios";

export const ITEM_PATH: string = '/api/v1/items';

export class ItemService {
    public static async parse<T>(
        content: string,
        blueprint: boolean = false
    ): Promise<T[]> {
        return (await axios.post(`${ITEM_PATH}/parse`, content, {
            params: {
                blueprint,
            },
            headers: {
                'Content-Type': 'application/json'
            }
        })).data;
    }

    public static async get(
        type_id: TypeId,
    ): Promise<IItem> {
        const stored = localStorage.getItem(<any> type_id);

        if (stored) {
            return JSON.parse(stored);
        }

        const result = (await axios.get(`${ITEM_PATH}/resolve/ids/${type_id}`)).data;

        localStorage.setItem(<any> type_id, JSON.stringify(result));
        return result;
    }

    public static async get_bulk(
        type_ids: TypeId[],
    ): Promise<IItem[]> {
        let cached: IItem[] = [];
        let remaining: TypeId[] = [];

        for(let type_id of type_ids) {
            const stored = localStorage.getItem(<any> type_id);

            if (stored) {
                cached.push(JSON.parse(stored));
            } else {
                remaining.push(type_id);
            }
        }

        if (remaining.length > 0) {
            const result = (await axios.post(`${ITEM_PATH}/resolve/ids`, remaining)).data;

            result
                .forEach((result: IItem) => {
                    localStorage.setItem(<any>result.type_id, JSON.stringify(result));
                    cached.push(result);
                });
        }

        return cached;
    }

    public static get_sync(
        type_id: TypeId,
    ): IItem {
        const stored = localStorage.getItem(<any> type_id);

        if (stored) {
            return JSON.parse(stored);
        } else {
            // it should be requested
            this.get(type_id);
            return {
                type_id,
                category_id: 0,
                group_id: 0,
                volume: 0,
                name: 'Unknown',
            };
        }
    }
}

export interface IItem {
    type_id:     TypeId;
    category_id: number;
    group_id:    number;
    volume:      number;
    name:        string;

    icon_url?:   string;
}
