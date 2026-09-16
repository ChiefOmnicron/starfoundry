import { ApplicationShell, type ApplicationShellRouteDefinition } from '@starfoundry/components/wrapper/ApplicationHarness';
import { createRootRouteWithContext } from '@tanstack/react-router';
import { MantineProvider } from '@mantine/core';
import { Route as AboutRoute } from '@/routes/about';
import { Route as IndexRoute } from '@/routes';
import { Route as LegalRoute } from '@/routes/legal';
import { THEME } from '@starfoundry/components/utils';
import type { ReactElement } from 'react';
import type { RouterContext } from '@/main';


const routes: ApplicationShellRouteDefinition[] = [{
    groups: [{
        link: '/industry-hubs',
        label: 'Industry Hubs',
        paths: [],
    }, {
        link: '/structures',
        label: 'Structures',
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
        name='StarFoundry - Industry Hubs'

        aboutRoute={AboutRoute.to}
        indexRoute={IndexRoute.to}
        legalRoute={LegalRoute.to}

        routes={routes}

        showSidebar={isAuthenticated}
        isAuthenticated={isAuthenticated ? 'AUTHENTICATED' : 'UNAUTHENTICATED'}
    />
}
