import axios from 'axios';
import type { StockBlueprintId, TypeId, Uuid } from '@/sdk/utils';

const STOCK_BLUEPRINT_PATH: string = '/api/v1/stocks/blueprints';

export class StockBlueprintService {
    public static cache: {
        [stockBlueprintId: StockBlueprintId]: StockBlueprint;
    } = {};

    public static async list(
        filter: IStockBlueprintFilter,
    ): Promise<StockBlueprint[]> {
        return await axios
            .get<StockBlueprintId[]>(STOCK_BLUEPRINT_PATH, {
                params: filter,
            })
            .then((x) => {
                let response: StockBlueprintId[] = [];
                if (x.data) {
                    response = x.data;
                }

                return Promise.allSettled(
                    response.map((y) => {
                        if (!this.cache) {
                            this.cache = {};
                        }

                        this.cache[y] = new StockBlueprint(y);
                        return this.cache[y].load();
                    }),
                );
            })
            .then((x) =>
                x
                    .filter((x) => x.status === 'fulfilled')
                    // TS does not understand that it has a value as it is
                    // definitly a successful promise
                    .map((x: any) => x.value),
            );
    }

    public static async fetch(
        stockBlueprintId: StockBlueprintId,
    ): Promise<StockBlueprint> {
        if (this.cache[stockBlueprintId]) {
            return this.cache[stockBlueprintId].load();
        }

        this.cache[stockBlueprintId] = new StockBlueprint(stockBlueprintId);
        return this.cache[stockBlueprintId].load();
    }

    public static async create(
        structure: IStockBlueprint,
    ): Promise<StockBlueprint> {
        return axios
            .post(`${STOCK_BLUEPRINT_PATH}`, structure)
            .then((x) => this.fetch(x.data));
    }
}

export class StockBlueprint {
    private loader: Promise<any> | null;

    private _info: IStockBlueprint = <any>null;
    private _thresholds: IStockBlueprintThreshold[] = [];

    public constructor(private _stockBlueprintId: StockBlueprintId) {
        this.loader = axios
            .get(`${STOCK_BLUEPRINT_PATH}/${this._stockBlueprintId}`)
            .then((x) => (this._info = x.data));
    }

    public async load(): Promise<StockBlueprint> {
        if (this.loader) {
            await this.loader;
            this.loader = null;
        }

        return this;
    }

    public async loadThresholds(): Promise<IStockBlueprintThreshold[]> {
        return await axios
            .get(`${STOCK_BLUEPRINT_PATH}/${this._stockBlueprintId}/thresholds`)
            .then((x) => {
                this._thresholds = x.data;
                return x.data;
            });
    }

    public async saveThresholds(): Promise<IStockBlueprintThreshold[]> {
        return await axios
            .put(
                `${STOCK_BLUEPRINT_PATH}/${this._stockBlueprintId}/thresholds`,
                this._thresholds,
            )
            .then((_) => this.loadThresholds());
    }

    public async save(): Promise<void> {
        return await axios.put(
            `${STOCK_BLUEPRINT_PATH}/${this._stockBlueprintId}`,
            this._info,
        );
    }

    public async remove(): Promise<void> {
        await axios.delete(`${STOCK_BLUEPRINT_PATH}/${this._stockBlueprintId}`);
        delete StockBlueprintService.cache[this._stockBlueprintId];
        return;
    }

    get id(): StockBlueprintId {
        return this._stockBlueprintId;
    }

    get name(): string {
        return this._info.name;
    }

    get description(): string {
        return this._info.description;
    }

    get notifications(): Uuid[] {
        return this._info.notifications;
    }

    set notifications(notifications: Uuid[]) {
        this._info.notifications = notifications;
    }

    get thresholds(): IStockBlueprintThreshold[] {
        return this._thresholds;
    }

    set thresholds(thresholds: IStockBlueprintThreshold[]) {
        this._thresholds = thresholds;
    }
}

export interface IStockBlueprintFilter {
    name?: string;
}

export interface IStockBlueprint {
    id?: Uuid;
    name: string;
    description: string;
    notifications: Uuid[];
}

export interface IStockBlueprintThreshold {
    id?: Uuid;
    type_id: TypeId;
    want: number;
    critical: number;
    min_runs: number;
    min_me: number;
    min_te: number;
}
