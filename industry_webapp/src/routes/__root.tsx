import { ApplicationShell, type ApplicationShellRouteDefinition } from '@starfoundry/components/wrapper/ApplicationHarness';
import { createRootRouteWithContext } from '@tanstack/react-router';
import { Route as AboutRoute } from '@/routes/about';
import { Route as IndexRoute } from '@/routes';
import { Route as LegalRoute } from '@/routes/legal';
import { THEME } from '@starfoundry/components/utils';
import type { ReactElement } from 'react';
import type { RouterContext } from '@/main';
import { MantineProvider } from '@mantine/core';

const routes: ApplicationShellRouteDefinition[] = [{
    groups: [{
            link: '/projects',
            label: 'Projects',
            subpath: '/projects/$projectId',
            paths: [{
                link: '/projects/$projectId/overview',
                label: 'Overview'
            }, {
                link: '/projects/$projectId/misc',
                label: 'Miscellaneous'
            }, {
                link: '/projects/$projectId/market',
                label: 'Market'
            }, {
                link: '/projects/$projectId/jobs',
                label: 'Jobs'
            }, {
                link: '/projects/$projectId/stock',
                label: 'Stock'
            }, {
                link: '/projects/$projectId/excess',
                label: 'Excess'
            }, {
                link: '/projects/$projectId/settings',
                label: 'Settings'
            }]
        }, {
            link: '/jobs',
            label: 'Industry Jobs',
            paths: [],
        }, {
            link: '/price-calculation',
            label: 'Price Calculation',
            paths: [],
        }, {
            link: '/project-groups',
            label: 'Project Groups',
            subpath: '/project-groups/$projectGroupId',
            paths: [{
                link: '/project-groups/$projectGroupId/overview',
                label: 'Overview'
            }, {
                link: '/project-groups/$projectGroupId/members',
                label: 'Members'
            }, {
                link: '/project-groups/$projectGroupId/industry-hubs',
                label: 'Industry Hubs'
            }, {
                link: '/project-groups/$projectGroupId/defaults',
                label: 'Defaults'
            }, {
                link: '/project-groups/$projectGroupId/settings',
                label: 'Settings'
            }]
        }]
    }, {
        groups: [{
            link: '/industry-hubs',
            label: 'Industry Hubs',
            paths: [],
        }, {
            link: '/structures',
            label: 'Structures',
            paths: [],
        }]
    }, {
        groups: [{
            link: '/tags',
            label: 'Tags',
            paths: [],
        }, {
            link: '/characters',
            label: 'Characters',
            paths: [],
        }]
    }];

export const Route = createRootRouteWithContext<RouterContext>()({
    component: RouteComponent,
    loader: async ({ context }) => {
        return {
            isAuthenticated: await context.auth.isAuthenticated()
        }
    }
});

function RouteComponent(): ReactElement {
    return <>
        <MantineProvider
            forceColorScheme='dark'
            theme={THEME}
        >
            {AppShell()}
        </MantineProvider>
    </>;
}

function AppShell() {
    const { isAuthenticated } = Route.useLoaderData();

    return <ApplicationShell
        name='StarFoundry - Industry'

        aboutRoute={AboutRoute.to}
        indexRoute={IndexRoute.to}
        legalRoute={LegalRoute.to}

        routes={routes}

        showSidebar={isAuthenticated}
        isAuthenticated={isAuthenticated ? 'AUTHENTICATED' : 'UNAUTHENTICATED'}
    />
}
