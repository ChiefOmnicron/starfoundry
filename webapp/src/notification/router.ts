export const ROUTE_NOTIFICATION = 'notification';
export const ROUTE_NOTIFICATIONS = 'notifications';
export const ROUTE_NOTIFICATION_CREATE = 'notification_create';

export default [
    {
        path: '/notifications/:notificationId',
        name: ROUTE_NOTIFICATION,
        props: true,
        component: () =>
            import(
                /* webpackChunkName: "notification_update" */
                '@/notification/update/View.vue'
            ),
    },
    {
        path: '/notifications',
        name: ROUTE_NOTIFICATIONS,
        component: () =>
            import(
                /* webpackChunkName: "notification_overview" */
                '@/notification/overview/View.vue'
            ),
    },
    {
        path: '/notifications/new',
        name: ROUTE_NOTIFICATION_CREATE,
        component: () =>
            import(
                /* webpackChunkName: "notification_create" */
                '@/notification/create/View.vue'
            ),
    },
];
