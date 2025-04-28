export const ROUTE_APPRAISAL: string = 'appraisal';
export const ROUTE_APPRAISAL_COMPRESSION: string = 'appraisal_compression';
export const ROUTE_APPRAISAL_EMPTY: string = 'appraisal_empty';
export const ROUTE_APPRAISAL_REPROCESSING: string = 'appraisal_reprocessing';

export default [
    {
        path: '/appraisal',
        name: ROUTE_APPRAISAL_EMPTY,
        component: () =>
            import(
                /* webpackChunkName: "appraisal" */
                '@/appraisal/Appraisal.vue'
            ),
    },
    {
        path: '/appraisal/:code',
        name: ROUTE_APPRAISAL,
        props: true,
        alias: [
            '/appraisal/:code/compression',
            '/appraisal/:code/reprocessing',
        ],
        component: () =>
            import(
                /* webpackChunkName: "appraisal_code" */
                '@/appraisal/Appraisal.vue'
            ),
    },
    //{
    //    path: '/appraisal/:code/compression',
    //    name: ROUTE_APPRAISAL_COMPRESSION,
    //    props: true,
    //    component: () => import(
    //        /* webpackChunkName: "appraisal" */
    //        '@/appraisal/Appraisal.vue'
    //    ),
    //},
    //{
    //    path: '/appraisal/:code/reprocessing',
    //    name: ROUTE_APPRAISAL_REPROCESSING,
    //    props: true,
    //    component: () => import(
    //        /* webpackChunkName: "appraisal" */
    //        '@/appraisal/Appraisal.vue'
    //    ),
    //},
];
