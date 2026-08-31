import { ApplicationShell } from '@starfoundry/components/wrapper/ApplicationHarness';
import { createRootRouteWithContext } from '@tanstack/react-router';
import { MantineProvider } from '@mantine/core';
import { Route as AboutRoute } from '@/routes/about';
import { Route as IndexRoute } from '@/routes';
import { Route as LegalRoute } from '@/routes/legal';
import { THEME } from '@starfoundry/components/utils';
import type { ReactElement } from 'react';
import type { RouterContext } from '@/main';

export const Route = createRootRouteWithContext<RouterContext>()({
    component: RouteComponent,
    //loader: async ({ context }) => {
    loader: async () => {
        return {
            //isAuthenticated: await context.auth.isAuthenticated()
            isAuthenticated: false,
        }
    }
});

function RouteComponent(): ReactElement {
    const { isAuthenticated } = Route.useLoaderData();

    return (
        <MantineProvider
            forceColorScheme='dark'
            theme={THEME}
        >
            <ApplicationShell
                name='StarFoundry - Appraisal'

                aboutRoute={AboutRoute.to}
                indexRoute={IndexRoute.to}
                legalRoute={LegalRoute.to}

                showSidebar={isAuthenticated}
                isAuthenticated={isAuthenticated ? 'AUTHENTICATED' : 'UNAUTHENTICATED'}
            />
        </MantineProvider>
    );
}
