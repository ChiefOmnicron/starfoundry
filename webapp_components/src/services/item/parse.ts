import { axiosClient } from "../client";
import type { Item } from "../item/model";
import type { GenericAbortSignal } from "axios";

export const parseItem = async (
    items:   string,
    signal?: GenericAbortSignal,
): Promise<Item[]> => (await axiosClient())
    .post(
        `/api/eve/items/parse`,
        items,
        {
            headers: {
                'Content-Type': 'application/json'
            },
            signal,
        },
    )
    .then(x => x.data.items.map((y: any) => y.raw));
