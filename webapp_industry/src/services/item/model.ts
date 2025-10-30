export type Item = {
    category:       Category;
    group:          Group;
    type_id:        number;
    name:           string;
    volume:         number;

    repackaged?:    number;
    meta_group_id?: number,
    base_price?:    number;
}

export type Category = {
    category_id:    number;
    name:           string;
};

export type Group = {
    group_id:       number;
    category_id:    number;
    name:           string;
};
