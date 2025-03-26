export const ROUTE_PROJECT_GROUP = 'project_group';
export const ROUTE_PROJECT_GROUP_CREATE = 'project_group_create';
export const ROUTE_PROJECT_GROUP_INVITE = 'project_group_invite';
export const ROUTE_PROJECT_GROUPS = 'project_groups';

export default [
    {
        path: '/projects/groups',
        name: ROUTE_PROJECT_GROUPS,
        component: () => import(
            /* webpackChunkName: "project_groups" */
            '@/project_group/List.vue'
        )
    },
    {
        path: '/projects/groups/create',
        name: ROUTE_PROJECT_GROUP_CREATE,
        component: () => import(
            /* webpackChunkName: "project_group_create" */
            '@/project_group/Create.vue'
        )
    },
    {
        path: '/projects/groups/:groupId',
        name: ROUTE_PROJECT_GROUP,
        props: true,
        component: () => import(
            /* webpackChunkName: "project_group" */
            '@/project_group/Group.vue'
        )
    },
    {
        path: '/projects/groups/:groupId/invite',
        name: ROUTE_PROJECT_GROUP_INVITE,
        props: true,
        component: () => import(
            /* webpackChunkName: "project_group_invite" */
            '@/project_group/Invite.vue'
        )
    },
];
