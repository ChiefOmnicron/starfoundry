export const ROUTE_TYPE_JUMP_ROUTE: RouteEntry = {
    label: 'Jump Route',
    value: 'JUMP_ROUTE',
};

export const ROUTE_TYPE_HAULING_ROUTE: RouteEntry = {
    label: 'Hauling Route',
    value: 'HAULING_ROUTE',
};

export const ROUTE_TYPE_HAULING_SERVICE: RouteEntry = {
    label: 'Hauling Service',
    value: 'HAULING_SERVICE',
};

export const ROUTE_TYPES: RouteEntry[] = [
    ROUTE_TYPE_JUMP_ROUTE,
    ROUTE_TYPE_HAULING_ROUTE,
    ROUTE_TYPE_HAULING_SERVICE,
];

export type RouteType = 'JUMP_ROUTE' | 'HAULING_ROUTE' | 'HAULING_SERVICE';

export type RouteEntry = {
    label: string;
    value: RouteType,
}
