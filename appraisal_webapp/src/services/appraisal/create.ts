import type { Item } from "@starfoundry/components/services/item/model";
import axios from "axios";

export const createAppraisal = async (
    data: CreateAppraisal,
): Promise<Appraisal> => axios
    .post(
        `/api/appraisals`,
        data,
    )
    .then(x => x.data);

export type CreateAppraisal = {
    item_str:   string;
    market_id:  number;
    mode:       AppraisalMode;
}

export type AppraisalMode = 'APPRAISAL' | 'MULTIBUY';

export type Appraisal = {
    code:           string,
    created_at_ts:  number;
    invalid:        string[];
    items:          AppraisalItem[];
    market_id:      number[];
    modifier:       number;
    comment?:       string;
    raw?:           string;
}

export type AppraisalItem = {
    source:             number;
    item:               Item,
    quantity:           number;
    insufficient_data:  boolean;
    last_fetch:         string;
    buy_price:          AppraisalItemPrice,
    sell_price:         AppraisalItemPrice,
}

export type AppraisalItemPrice = {
    max:            number;
    min:            number;
    total_orders:   number;
}
