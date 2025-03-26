import axios from "axios";
import type { NotificationId, Uuid } from "@/sdk/utils";

const NOTIFICATIONS_PATH: string = '/api/v1/notifications';

export class NotificationService {
    public static cache: { [notificationId: NotificationId]: Notification } = {};

    public static async list(
        filter: INotificationFilter,
    ): Promise<Notification[]> {
        return await axios
            .get<NotificationId[]>(NOTIFICATIONS_PATH, {
                params: filter,
            })
            .then(x => {
                let response: NotificationId[] = [];
                if (x.data) {
                    response = x.data;
                }

                return Promise.allSettled(
                    response
                        .map(y => {
                            if (!this.cache) {
                                this.cache = {};
                            }

                            this.cache[y] = new Notification(y);
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
        notificationId: NotificationId,
    ): Promise<Notification> {
        if (this.cache[notificationId]) {
            return this.cache[notificationId].load();
        }

        this.cache[notificationId] = new Notification(notificationId);
        return this.cache[notificationId].load();
    }

    public static async create(
        notification: INotification,
    ): Promise<Notification> {
        return axios
            .post(`${NOTIFICATIONS_PATH}`, notification)
            .then(x => this.fetch(x.data));
    }

    public static async testMessage(
        target: string,
        url:    string,
    ): Promise<string> {
        return await axios
            .post(`${NOTIFICATIONS_PATH}/test-message`, {
                target,
                url
            })
            .then(x => {
                console.log(x)
                return x.data;
            })
            .catch(x => {
                console.log(x.response.data)
                throw new Error(x.response.data);
            });
    }
}

export class Notification {
    private loader: Promise<any> | null;

    private _info: INotification = <any>null;

    public constructor(
        private _notificationId: NotificationId,
    ) {
        this.loader = axios
            .get(`${NOTIFICATIONS_PATH}/${this._notificationId}`)
            .then(x => this._info = x.data);
    }

    public async load(): Promise<Notification> {
        if (this.loader) {
            await this.loader;
            this.loader = null;
        }

        return this;
    }

    public async save(): Promise<void> {
        return await axios
            .put(`${NOTIFICATIONS_PATH}/${this._notificationId}`, this._info);
    }

    public async remove(): Promise<void> {
        await axios.delete(`${NOTIFICATIONS_PATH}/${this._notificationId}`);
        delete NotificationService.cache[this._notificationId];
        return;
    }

    get id(): NotificationId {
        return this._notificationId;
    }

    get name(): string {
        return this._info.name
    }

    set name(name: string) {
        this._info.name = name;
    }

    get target(): WebhookTarget {
        return this._info.target;
    }

    set target(target: WebhookTarget) {
        this._info.target = target;
    }

    get url(): string {
        return this._info.url;
    }

    set url(url: string) {
        this._info.url = url;
    }
}

export interface INotificationFilter {
    name?:   string;
    target?: string;
}

export interface INotification {
    id?:    Uuid;
    name:   string;
    url:    string;
    target: WebhookTarget;
}

export interface INotificationTestMessage {
    status:  string;
    message: string;
}

export type WebhookTarget = 'DISCORD' | 'JSON';
