export type Item = {
    category_id: number;
    group_id: number;
    type_id: number;
    name: string;
    volume: number;

    repackaged?: number;
    meta_group_id?: number,
    base_price?: number;
}
