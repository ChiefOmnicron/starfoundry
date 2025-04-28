export const ROUTE_JOB_DETECTION_LIST = 'job_detection_list';

export default [
    {
        path: '/job-detection',
        name: ROUTE_JOB_DETECTION_LIST,
        component: () =>
            import(
                /* webpackChunkName: "job_detection_list" */
                '@/job_detection/List.vue'
            ),
    },
];
