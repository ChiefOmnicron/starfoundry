export const ROUTE_PROJECT_ASSIGNMENTS = 'project_job_assignments';
export const ROUTE_PROJECT_CREATE = 'project_create';
export const ROUTE_PROJECT_EXCESS = 'project_excess';
export const ROUTE_PROJECT_JOB = 'project_jobs';
export const ROUTE_PROJECT_MARKET = 'project_market';
export const ROUTE_PROJECT_MISC = 'project_misc';
export const ROUTE_PROJECT_OVERVIEW = 'project_overview';
export const ROUTE_PROJECT_READY_JOBS = 'project_ready_jobs';
export const ROUTE_PROJECT_SETTINGS = 'project_settings';
export const ROUTE_PROJECT_STATISTICS = 'project_statistics';
export const ROUTE_PROJECT_STOCK = 'project_stock';
export const ROUTE_PROJECT_ACTIVE_JOBS = 'project_active_jobs';
export const ROUTE_PROJECTS = 'projects';

export default [
    {
        path: '/projects',
        name: ROUTE_PROJECTS,
        component: () => import(
            /* webpackChunkName: "projects" */
            '@/project/List.vue'
        )
    },
    {
        path: '/projects/create',
        name: ROUTE_PROJECT_CREATE,
        component: () => import(
            /* webpackChunkName: "project_create" */
            '@/project/Create.vue'
        )
    },
    {
        path: '/projects/ready',
        name: ROUTE_PROJECT_READY_JOBS,
        component: () => import(
            /* webpackChunkName: "project_ready" */
            '@/project/ReadyJobs.vue'
        )
    },
    {
        path: '/projects/:projectId',
        name: ROUTE_PROJECT_OVERVIEW,
        props: true,
        component: () => import(
            /* webpackChunkName: "project_overview" */
            '@/project/Overview.vue'
        )
    },
    {
        path: '/projects/:projectId/jobs',
        name: ROUTE_PROJECT_JOB,
        props: true,
        component: () => import(
            /* webpackChunkName: "project_jobs" */
            '@/project/Jobs.vue'
        )
    },
    {
        path: '/projects/:projectId/jobs/active',
        name: ROUTE_PROJECT_ACTIVE_JOBS,
        props: true,
        component: () => import(
            /* webpackChunkName: "project_active_jobs" */
            '@/project/job/ActiveJobs.vue'
        )
    },
    {
        path: '/projects/:projectId/market',
        name: ROUTE_PROJECT_MARKET,
        props: true,
        component: () => import(
            /* webpackChunkName: "project_market" */
            '@/project/Market.vue'
        )
    },
    {
        path: '/projects/:projectId/misc',
        name: ROUTE_PROJECT_MISC,
        props: true,
        component: () => import(
            /* webpackChunkName: "project_misc" */
            '@/project/Misc.vue'
        )
    },
    {
        path: '/projects/:projectId/stocks',
        name: ROUTE_PROJECT_STOCK,
        props: true,
        component: () => import(
            /* webpackChunkName: "project_stocks" */
            '@/project/Stock.vue'
        )
    },
    {
        path: '/projects/:projectId/settings',
        name: ROUTE_PROJECT_SETTINGS,
        props: true,
        component: () => import(
            /* webpackChunkName: "project_settings" */
            '@/project/Settings.vue'
        )
    },
    {
        path: '/projects/:projectId/excess',
        name: ROUTE_PROJECT_EXCESS,
        props: true,
        component: () => import(
            /* webpackChunkName: "project_excess" */
            '@/project/Excess.vue'
        )
    },
    {
        path: '/projects/statistics',
        name: ROUTE_PROJECT_STATISTICS,
        component: () => import(
            /* webpackChunkName: "statistics" */
            '@/project/Statistics.vue'
        )
    },
    {
        path: '/projects/jobs/assignments/:assignment_id',
        name: ROUTE_PROJECT_ASSIGNMENTS,
        component: () => import(
            /* webpackChunkName: "job_assignments" */
            '@/project/JobAssignments.vue'
        ),
        meta: {
            noAuth: true,
        },
    },
];
