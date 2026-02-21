import { type ReactElement } from "react";
export declare function Filter({ entries, onFilterChange, selectedFilter, }: FilterProp): ReactElement;
export type FilterProp = {
    entries: FilterPropEntry[];
    onFilterChange: (filters: SelectedFilter[]) => void;
    selectedFilter?: SelectedFilter[];
};
export type FilterPropEntry = {
    label: string;
    key: string;
    type: 'STRING' | 'SELECT' | 'MULTISELECT';
    options?: FilterPropOption[];
};
export type FilterPropOption = {
    label: string;
    key: string | number;
};
export type SelectedFilter = {
    filterLabel: string;
    filterKey: string;
    value: number | string | Array<string>;
    key: number | string;
};
