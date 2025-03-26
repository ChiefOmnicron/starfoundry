export const ROUTE_INDUSTRY_INDEX = 'industry_index';
export const ROUTE_INDUSTRY_JOBS  = 'industry_jobs';
export const ROUTE_COST_ESTIMATE  = 'industry_cost_estimate';

export default [
    {
        path: '/industry/index',
        name: ROUTE_INDUSTRY_INDEX,
        component: () => import(
            /* webpackChunkName: "system_index" */
            '@/industry/SystemIndex.vue'
        )
    },
    {
        path: '/industry/jobs',
        name: ROUTE_INDUSTRY_JOBS,
        component: () => import(
            /* webpackChunkName: "industry_jobs" */
            '@/industry/IndustryJobs.vue'
        )
    },
    {
        path: '/industry/estimate',
        name: ROUTE_COST_ESTIMATE,
        component: () => import(
            /* webpackChunkName: "industry_cost_estimate" */
            '@/industry/CostEstimate.vue'
        )
    },
];
