import { axiosClient } from "@internal/services/client";
import type { Item } from "@internal/services/item/model";
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
