export const ROUTE_STOCK_BLUEPRINT = 'stock_blueprint';
export const ROUTE_STOCK_BLUEPRINTS = 'stock_blueprints';
export const ROUTE_STOCK_BLUEPRINT_CREATE = 'stock_blueprint_create';

export default [
    {
        path: '/stocks/blueprints/:stockBlueprintId',
        name: ROUTE_STOCK_BLUEPRINT,
        props: true,
        component: () =>
            import(
                /* webpackChunkName: "stock_blueprint_update" */
                '@/stock/blueprint/update/View.vue'
            ),
    },
    {
        path: '/stock/blueprints',
        name: ROUTE_STOCK_BLUEPRINTS,
        component: () =>
            import(
                /* webpackChunkName: "stock_blueprint_list" */
                '@/stock/blueprint/list/View.vue'
            ),
    },
    {
        path: '/stock/blueprints/new',
        name: ROUTE_STOCK_BLUEPRINT_CREATE,
        component: () =>
            import(
                /* webpackChunkName: "stock_blueprint_create" */
                '@/stock/blueprint/create/View.vue'
            ),
    },
];
