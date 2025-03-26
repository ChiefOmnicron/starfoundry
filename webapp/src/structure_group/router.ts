export const ROUTE_STRUCTURE_GROUP             = 'structure_group';
export const ROUTE_STRUCTURE_GROUPS            = 'structure_groups';
export const ROUTE_STRUCUTRE_GROUP_NEW         = 'structure_group_new';
export const ROUTE_STRUCUTRE_GROUP_DYNAMIC_NEW = 'structure_group_dynamic_create';

export default [
  {
    path: '/structures/groups/:structureGroupId',
    name: ROUTE_STRUCTURE_GROUP,
    component: () => import(
      /* webpackChunkName: "structure_group" */
      '@/structure_group/Overview.vue'
    )
  },
  {
    path: '/structures/groups',
    name: ROUTE_STRUCTURE_GROUPS,
    component: () => import(
      /* webpackChunkName: "structure_groups" */
      '@/structure_group/Overview.vue'
    )
  },
  {
    path: '/structures/groups/new',
    name: ROUTE_STRUCUTRE_GROUP_NEW,
    component: () => import(
      /* webpackChunkName: "structure_group_new" */
      '@/structure_group/Create.vue'
    )
  },
];
