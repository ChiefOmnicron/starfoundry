import { useQuery } from "@tanstack/react-query";
import axios from "axios";
import type { GenericAbortSignal } from "axios";
import type { Item } from "../item/model";
import type { AbortSignal } from "../axiosClient";

export const FETCH_APPRAISAL = 'fetchAppraisal';

export const fetchAppraisal = async (
    code: string,
    signal?: GenericAbortSignal,
): Promise<Appraisal> => axios
    .get(
        `/api/appraisals/${code}`,
        {
            signal,
        }
    )
    .then(x => {
        console.log(x)
        return x.data
    })
    .catch(e => {
        if (e.status === 404) {
            throw new Error("Error 404")
        }
    });

// For general use
export const useFetchAppraisal = (
    code: string,
) => {
    return useQuery(fetchAppraisalQuery(code));
}

// For pre-fetching
export const fetchAppraisalQuery = (
    code: string,
) => ({
    queryKey: [FETCH_APPRAISAL, code],
    queryFn: async ({
        signal,
    }: AbortSignal) => fetchAppraisal(code, signal),
    // ms * s * m
    staleTime: 1000 * 60 * 5,
});


export type AppraisalMode = 'APPRAISAL' | 'MULTIBUY';

export type Appraisal = {
    code:           string,
    created_at_ts:  number;
    invalid:        string[];
    items:          AppraisalItem[];
    market_id:      number;
    modifier:       number;
    mode:           AppraisalMode,
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
