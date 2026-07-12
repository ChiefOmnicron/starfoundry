import { axiosClient, type AbortSignal } from "@internal/services/client";
import { useQuery } from "@tanstack/react-query";
import type { GenericAbortSignal } from "axios";
import type { Item } from "@internal/services/item/model";
import type { ProjectGroup } from "@internal/services/project-group/fetch";
import type { ProjectProduct } from "@internal/services/projects/listProduct";
import type { ProjectStatus } from "@internal/services/projects/list";
import type { Tag } from "@internal/services/tags/list";
import type { Uuid } from "@internal/services/utils";

export const FETCH_PROJECT = 'fetchProject';

export const fetchProject = async (
    projectId: Uuid,
    signal?:   GenericAbortSignal,
): Promise<ProjectList> => (await axiosClient())
    .get(
        `/api/projects/${projectId}`,
        {
            signal,
        }
    )
    .then(x => x.data);

// For general use
export const useFetchProject = (
    id: Uuid,
) => {
    return useQuery(fetchProjectQuery(id));
}

// For pre-fetching
export const fetchProjectQuery = (
    id: Uuid,
) => ({
    queryKey: [FETCH_PROJECT, id],
    queryFn: async ({
        signal
    }: AbortSignal) => fetchProject(id, signal),
    // ms * s * m
    staleTime: 1000 * 60 * 5,
});

export type ProjectList = {
    id:                 Uuid;
    name:               string;
    status:             ProjectStatus;
    orderer:            string;
    sell_price:         number;
    project_group:      ProjectGroup;
    products:           ProjectProduct[];
    stock:              ProjectStock[];
    excess:             ProjectExcess[];
    tags:               Tag[];

    note?:              string;

    pre_products?:      string;
    pre_additional?:    string;
}

export type ProjectStock = {
    item:       Item;
    quantity:   number;
    cost:       number;
}


export type ProjectExcess = {
    item:       Item;
    quantity:   number;
    cost:       number;
}
