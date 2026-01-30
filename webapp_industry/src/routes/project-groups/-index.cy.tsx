import { createRootRoute, createRouter, RouterProvider } from "@tanstack/react-router";
import { MantineProvider } from "@mantine/core";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { Route } from './index';

import '@mantine/core/styles.css';

const queryClient = new QueryClient({
    defaultOptions: {
        dehydrate: {
            shouldDehydrateQuery: () => false
        },
        queries: {
            retry: false,
        }
    }
});

const componentMount = () => {
    const routeTree = createRootRoute({
        component: Route.options.component,
    });
    const router = createRouter({
        routeTree,
        defaultStaleTime: 0,
        scrollRestoration: true,
    });
    return(
        <MantineProvider>
            <QueryClientProvider client={queryClient}>
                <RouterProvider router={router} />
            </QueryClientProvider>
        </MantineProvider>
    );
}

describe('Project Group General', () => {
    it('should show the loading animation', () => {
        cy.intercept(
            {
                method: 'GET',
                url: '/api/project-groups',
            },
            (req) => {
                // do nothing with the req, only call the response with a 10s delay.
                req.continue((res) => {
                    res.delay = 10000;
                    res.send();
                });
            },
        ).as('getTimeoutProjectGroup');

        cy.mount(componentMount());

        cy.get('[data-cy="loading"]').should('be.visible');

        cy.get('[data-cy="error"]').should('not.exist');
        cy.get('[data-cy="noData"]').should('not.exist');
        cy.get('[data-cy="data"]').should('not.exist');
    });

    it('should show the error message', () => {
        cy.intercept(
            {
                method: 'GET',
                url: '/api/project-groups',
            },
            {
                statusCode: 401,
            },
        ).as('getErrorProjectGroup');

        cy.mount(componentMount());

        cy.wait('@getErrorProjectGroup');

        cy.get('[data-cy="error"]').should('be.visible');

        cy.get('[data-cy="loading"]').should('not.exist');
        cy.get('[data-cy="noData"]').should('not.exist');
        cy.get('[data-cy="data"]').should('not.exist');
    });

    it('should show that there are no project groups yet', () => {
        cy.intercept(
            {
                method: 'GET',
                url: '/api/project-groups',
            },
            [],
        ).as('getEmptyProjectGroup');

        cy.mount(componentMount());

        cy.wait('@getEmptyProjectGroup');

        cy.get('[data-cy="noData"]').should('be.visible');
        
        cy.get('[data-cy="loading"]').should('not.exist');
        cy.get('[data-cy="error"]').should('not.exist');
        cy.get('[data-cy="data"]').should('not.exist');
    });

    it('should show the table', () => {
        cy.intercept(
            {
                method: 'GET',
                url: '/api/project-groups',
            },
            [{
                name: 'Test',
                member: 1,
                projects: 0,
                description: 'Test 123'
            }],
        ).as('getRoute.options.component');

        cy.mount(componentMount());

        cy.wait('@getRoute.options.component');

        cy.get('[data-cy="data"]').should('be.visible');

        cy.get('[data-cy="loading"]').should('not.exist');
        cy.get('[data-cy="error"]').should('not.exist');
        cy.get('[data-cy="noData"]').should('not.exist');
    });

    it('show delete successful message', () => {
        cy.intercept(
            {
                method: 'GET',
                url: '/api/project-groups',
            },
            [{
                name: 'Test',
                member: 1,
                projects: 0,
                description: 'Test 123'
            }],
        ).as('getRoute.options.component');

        const componentMount = () => {
            const routeTree = createRootRoute({
                component: Route.options.component,
                beforeLoad: ({ search }: { search: any }) => {
                    search.deleted = true;
                }
            });
            const router = createRouter({
                routeTree,
                defaultStaleTime: 0,
                scrollRestoration: true,
            });
            return(
                <MantineProvider>
                    <QueryClientProvider client={queryClient}>
                        <RouterProvider router={router} />
                    </QueryClientProvider>
                </MantineProvider>
            );
        }
        cy.mount(componentMount());

        cy.wait('@getRoute.options.component');
        cy.get('[data-cy="deleteSuccessful"]').should('be.visible');
    });
});

