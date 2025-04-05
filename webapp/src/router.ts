import { createRouter, createWebHistory, type RouteRecordRaw } from 'vue-router'

import { PROJECT_ROUTE, ROUTE_CHANGE } from '@/event_bus';
import { events } from '@/main'

import appraisalRoutes from '@/appraisal/router';
import characterRoutes from '@/characters/router';
import industryRoutes from '@/industry/router';
import jobDetectionRoutes from '@/job_detection/router';
import notificationRoutes from '@/notification/router';
import projectRoutes, { ROUTE_PROJECT_ASSIGNMENTS, ROUTE_PROJECT_CREATE, ROUTE_PROJECT_READY_JOBS } from '@/project/router';
import projectGroupRoutes, { ROUTE_PROJECT_GROUP, ROUTE_PROJECT_GROUP_CREATE, ROUTE_PROJECT_GROUP_INVITE, ROUTE_PROJECT_GROUPS } from '@/project_group/router';
import stockRoutes from '@/stock/router';
import structureDynamicGroupsRoutes from '@/structure_dynamic_group/router';
import structureGroupsRoutes from '@/structure_group/router';
import structureRoutes from '@/structure/router';

export const ROUTE_ABOUT = 'about';

let router;

// #v-ifdef VITE_APPRAISAL
const routesAppraisal: Array<RouteRecordRaw> = [
    ...appraisalRoutes,
    {
        path: '/',
        redirect: '/appraisal'
    },
    {
        path: '/about',
        name: 'about',
        component: () => import(
            /* webpackChunkName: "about-appraisal" */
            '@/AboutAppraisal.vue'
        )
    },
    {
        path: '/legal',
        name: 'legal',
        component: () => import(
            /* webpackChunkName: "legal" */
            '@/Legal.vue'
        )
    },
]

router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes: routesAppraisal,
});
// #v-endif

// #v-ifdef VITE_APPRAISAL==false
const routesIndustry: Array<RouteRecordRaw> = [
    ...appraisalRoutes,
    ...characterRoutes,
    ...industryRoutes,
    ...jobDetectionRoutes,
    ...notificationRoutes,
    ...projectGroupRoutes,
    ...projectRoutes,
    ...stockRoutes,
    ...structureDynamicGroupsRoutes,
    ...structureGroupsRoutes,
    ...structureRoutes,
    {
        path: '/',
        redirect: '/projects'
    },
    {
        path: '/about',
        name: 'about',
        component: () => import(
            /* webpackChunkName: "about" */
            '@/About.vue'
        )
    },
    {
        path: '/legal',
        name: 'legal',
        component: () => import(
            /* webpackChunkName: "legal" */
            '@/Legal.vue'
        )
    },
]

router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes: routesIndustry,
});

router.afterEach((to, _) => {
    if (
        to &&
        to.name &&
        to?.name?.toString().indexOf('project') >= 0 &&
        [
            ROUTE_PROJECT_ASSIGNMENTS,
            ROUTE_PROJECT_READY_JOBS,

            ROUTE_PROJECT_CREATE,

            ROUTE_PROJECT_GROUP,
            ROUTE_PROJECT_GROUP_CREATE,
            ROUTE_PROJECT_GROUP_INVITE,
            ROUTE_PROJECT_GROUPS,
        ].indexOf(to?.name?.toString()) === -1
    ) {
        events.$emit(PROJECT_ROUTE, to.name.toString());
    } else {
        document.title = 'StarFoundry';
        events.$emit(ROUTE_CHANGE, to?.name?.toString());
    }
});
// #v-endif

export default router;
