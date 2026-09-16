import axios from "axios";
import type { Appraisal, AppraisalMode } from "./fetch";

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
