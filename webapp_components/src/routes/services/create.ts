import { axiosClient } from "@internal/services/client";
import type { RouteType} from '@internal/routes/services/options';
import type { Uuid } from "@internal/services/utils";

export const createTag = async (
    data: CreateRoute,
): Promise<CreateRouteResponse> => (await axiosClient())
    .post(
        '/api/routes',
        data,
    )
    .then(x => x.data);

export type CreateRoute = {
    name:               string;
    typ:                RouteType;

    start_structure:    Uuid;
    end_structure:      Uuid;

    jump_route?:        CreateRouteJumpRoute;
    hauling_route?:     CreateRouteHaulingRoute;
    hauling_service?:   CreateRouteHaulingService;
}

export type CreateRouteHaulingService = {
    contract_to:        string;
    price_per_m3:       number;
    max_cargo_m3:       number;
    collateral_percent: number;
}

export type CreateRouteHaulingRoute = {
    fuel_usage:                 number;
    max_cargo_m3:               number;
}

export type CreateRouteJumpRoute = {
    fuel_usage:                 number;
}

export type CreateRouteResponse = {
    id: Uuid,
}
