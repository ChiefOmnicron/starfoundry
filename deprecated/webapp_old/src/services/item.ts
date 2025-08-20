import type { TypeId } from '@/utils';
import axios from 'axios';

export class ItemService {
    private static cache: { [key: TypeId]: IItem } = {};

    public static async all(): Promise<IItem[]> {
        return (await axios.get('/api/v1/items')).data;
    }

    public static async buildable_items(): Promise<IItem[]> {
        return (await axios.get('/api/v1/items/buildable')).data;
    }

    public static async blueprints(): Promise<IItem[]> {
        return (await axios.get('/api/v1/items/blueprints')).data;
    }

    public static async resolve_id(tid: TypeId): Promise<IItem> {
        if (this.cache[tid]) {
            return this.cache[tid];
        } else {
            let entry: IItem = (await axios.get(`/api/v1/items/resolve/${tid}`))
                .data;
            this.cache[entry.type_id] = entry;
            return entry;
        }
    }

    // TODO: add buildable
    public static async resolve_names_bulk(
        names: string[],
    ): Promise<IBulkItem[]> {
        return (
            await axios.post<IBulkItem[]>(`/api/v1/items/resolve/names`, names)
        ).data;
    }

    public static async resolve_id_from_name_bulk(
        names: string[],
        params: { [key: string]: any },
    ): Promise<number[]> {
        return (await axios.post('/api/v1/items/resolve', names, { params }))
            .data;
    }

    public static async parse<T>(
        content: string,
        blueprint: boolean = false,
    ): Promise<T[]> {
        return (
            await axios.post('/api/v1/items/parse', content, {
                params: {
                    blueprint,
                },
                headers: {
                    'Content-Type': 'application/json',
                },
            })
        ).data;
    }

    public static async resolve_names_from_rows(
        rows: string,
    ): Promise<IParsedRow[]> {
        let name_quantity = new Map();
        let lines = rows.split('\n');

        for (let line of lines) {
            let row = line.replaceAll('\t', ' ').split(' ');
            if (!isNaN(<any>row[row.length - 1])) {
                let quantity = row.pop() || '1';
                name_quantity.set(row.join(' '), parseInt(quantity));
            } else {
                name_quantity.set(row.join(' '), 1);
            }
        }

        let ids = await ItemService.resolve_names_bulk(
            Array.from(name_quantity.keys()),
        );

        let resolved_names = [];
        for (let id of ids) {
            let quantity = name_quantity.get(id.name);
            resolved_names.push({
                quantity: quantity,
                type_id: id.type_id,
            });
        }

        return resolved_names;
    }
}

export interface IItem {
    type_id: number;
    category_id: number;
    group_id: number;
    volume: number;
    name: string;

    icon_url?: string;
}

export interface IBulkItem {
    type_id: number;
    name: string;
    base_price: number;
}

export interface IParsedRow {
    quantity: number;
    type_id: number;
}
