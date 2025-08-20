export const ROUTE_STRUCTURE = 'structure';
export const ROUTE_STRUCTURES = 'structures';
export const ROUTE_STRUCUTRES_CREATE = 'structures_create';

export default [
    {
        path: '/structures/:structureId',
        name: ROUTE_STRUCTURE,
        props: true,
        component: () =>
            import(
                /* webpackChunkName: "structure_update" */
                '@/structure/update/View.vue'
            ),
    },
    {
        path: '/structures',
        name: ROUTE_STRUCTURES,
        component: () =>
            import(
                /* webpackChunkName: "structures_overview" */
                '@/structure/overview/View.vue'
            ),
    },
    {
        path: '/structures/new',
        name: ROUTE_STRUCUTRES_CREATE,
        component: () =>
            import(
                /* webpackChunkName: "structure_create" */
                '@/structure/create/View.vue'
            ),
    },
];
