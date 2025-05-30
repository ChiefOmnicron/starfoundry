export const ROUTE_STRUCTURE_DYNAMIC_GROUP = 'structure_dynamic_group';
export const ROUTE_STRUCTURE_DYNAMIC_GROUPS = 'structure_dynamic_groups';
export const ROUTE_STRUCUTRE_GROUP_NEW = 'structure_dynamic_group_new';

export default [
    {
        path: '/structures/groups/dynamic',
        name: ROUTE_STRUCTURE_DYNAMIC_GROUPS,
        component: () =>
            import(
                /* webpackChunkName: "structure_dynamic_groups" */
                '@/structure_dynamic_group/Overview.vue'
            ),
    },
    {
        path: '/structures/groups/dynamic/new',
        name: ROUTE_STRUCUTRE_GROUP_NEW,
        component: () =>
            import(
                /* webpackChunkName: "structure_dynamic_group_new" */
                '@/structure_dynamic_group/Create.vue'
            ),
    },
];
