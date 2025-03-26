import axios from "axios";

export async function createAppraisal(
    appraisal:      string,
    market_id:      number,
    comment:        string | undefined,
    price_modifier: number,
): Promise<IAppraisal> {
    return axios
        .post<IAppraisal>(
            `/api/v1/appraisals`,
            {
                appraisal,
                market_id,
                comment,
                price_modifier,
            },
            {
                headers: {
                    'Content-Type': 'application/json',
                }
            }
        )
        .then(x => x.data)
}

export async function fetchAppraisal(
    code: string,
): Promise<IAppraisal> {
    return axios
        .get<IAppraisal>(`/api/v1/appraisals/${code}`)
        .then(x => x.data)
}

export async function fetchAppraisalCompression(
    code:               string,
    compressionOptions: ICompressionOptions,
): Promise<ICompressionResult> {
    return axios
        .put<ICompressionResult>(
            `/api/v1/appraisals/${code}/compression`,
            compressionOptions
        )
        .then(x => x.data)
}

export async function fetchAppraisalReprocessing(
    code:                string,
    reprocessingOptions: IReprocessingOptions,
): Promise<IAppraisal> {
    return axios
        .put<IAppraisal>(
            `/api/v1/appraisals/${code}/reprocessing`,
            reprocessingOptions
        )
        .then(x => x.data)
}

export interface ICompressionResult {
    compression_appraisal: IAppraisal;
    overage_appraisal?:    IAppraisal;
}

export interface IAppraisal {
    created_at:     number;

    market_id:         number;

    code:           string;
    items:          IAppraisalItem[];
    invalid:        string[];

    comment?:       string;
    price_modifier: number;
}

export interface IAppraisalItem {
    quantity: number;
    type_id:  number;

    meta:     IItem;

    buy:      IMarketEntry;
    sell:     IMarketEntry;

    low_data: boolean;
}

export interface IItem {
    name:           string;
    volume:         number;

    category_id:    number;
    group_id:       number;
    type_id:        number;
    meta_group_id?: number;
    repackaged?:    number;
}

export interface IMarketEntry {
    max:    number;
    min:    number;

    total_orders: number;

    per_item:  IMarketEntryPerItem;
}

export interface IMarketEntryPerItem {
    avg:    number;
    max:    number;
    min:    number;
}

export interface ICompressionOptions {
    ore_reprocessing:        OreReprocessing;
    gas_decompression:       GasDecompression;

    allow_minerals:          boolean;
    allow_uncompressed_gas:  boolean;
    allow_compressed_moon:   boolean;
    allow_uncompressed_moon: boolean;
    allow_uncompressed_ore:  boolean;

    blacklist:               number[];
}

export function defaultCompressionOptions(): ICompressionOptions {
    return {
        ore_reprocessing:        'NsTataraT2',
        gas_decompression:       'TataraLvl5',

        allow_minerals:          false,
        allow_uncompressed_gas:  false,
        allow_compressed_moon:   false,
        allow_uncompressed_moon: false,
        allow_uncompressed_ore:  false,

        blacklist:               [],
    }
}

export interface IReprocessingOptions {
    ore_reprocessing:   OreReprocessing;
    gas_decompression:  GasDecompression;
    scrap_reprocessing: ScrapReprocessing;
}

export function defaultReprocessingOptions(): IReprocessingOptions {
    return {
        ore_reprocessing:   'NsTataraT2',
        gas_decompression:  'TataraLvl5',
        scrap_reprocessing: 'Lvl5'
    }
}

export type OreReprocessing = 'HsAthanorNoRig' |
    'HsAthanorT1' |
    'HsAthanorT2' |
    'HsTataraNoRig' |
    'HsTataraT1' |
    'HsTataraT2' |

    'LsAthanorNoRig' |
    'LsAthanorT1' |
    'LsAthanorT2' |
    'LsTataraNoRig' |
    'LsTataraT1' |
    'LsTataraT2' |

    'NsAthanorNoRig' |
    'NsAthanorT1' |
    'NsAthanorT2' |
    'NsTataraNoRig' |
    'NsTataraT1' |
    'NsTataraT2';
export type GasDecompression = 'AthanorLvl0' |
    'AthanorLvl1' |
    'AthanorLvl2' |
    'AthanorLvl3' |
    'AthanorLvl4' |
    'AthanorLvl5' |

    'TataraLvl0' |
    'TataraLvl1' |
    'TataraLvl2' |
    'TataraLvl3' |
    'TataraLvl4' |
    'TataraLvl5'

export type ScrapReprocessing = 'Lvl0' |
    'Lvl1' |
    'Lvl2' |
    'Lvl3' |
    'Lvl4' |
    'Lvl5'
