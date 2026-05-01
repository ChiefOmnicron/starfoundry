import { axiosClient } from "@internal/services/client";

export const createFitting = async (
    characterId:    number,
    data:           EveFit,
): Promise<void> => (await axiosClient())
    .post(
        `/api/eve/eve/characters/${characterId}/fittings`,
        data,
    )
    .then(x => x.data);

export type EveFit = {
    name:           string;
    description:    string;
    items:          EveFitItem[];
    ship_type_id:   number;
}

export type EveFitItem = {
    quantity:   number;
    type_id:    number;
    flag:       'Cargo' | 'LoSlot0' | 'LoSlot1' | 'LoSlot2';
}
