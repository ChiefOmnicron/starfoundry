export const ROUTE_APPRAISAL: string = 'appraisal';
export const ROUTE_APPRAISAL_EMPTY: string = 'appraisal_empty';

export default [
    {
        path: '/appraisal',
        name: ROUTE_APPRAISAL_EMPTY,
        component: () => import(
            /* webpackChunkName: "appraisal" */
            '@/appraisal/Appraisal.vue'
        ),
    },
    {
        path: '/appraisal/:code',
        name: ROUTE_APPRAISAL,
        props: true,
        component: () => import(
            /* webpackChunkName: "appraisal" */
            '@/appraisal/Appraisal.vue'
        ),
    },
]
