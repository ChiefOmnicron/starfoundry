import { createRootRoute, createRouter, RouterProvider } from "@tanstack/react-router";
import { MantineProvider } from "@mantine/core";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { ProjectGroupHeader } from './$projectGroupId.route';

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
        component: ProjectGroupHeader,
        beforeLoad: ({ params }: { params: any }) => {
            params.projectGroupId = 'e52744df-a140-4e72-b24c-18f534d0ae94';
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

describe('Project Group Header', () => {
    it('makes sure that everything is loaded and correctly shown', () => {
        cy.intercept(
            {
                method: 'GET',
                url: '/api/project-groups/e52744df-a140-4e72-b24c-18f534d0ae94',
            },
            (req) => {
                req.reply({
                    body: {
                        id: 'e52744df-a140-4e72-b24c-18f534d0ae94',
                        name: 'Test',
                        members: 1,
                        projects: 0,
                        is_owner: true,

                        description: 'Some description',
                    },
                    delay: 1000,
                    statusCode: 200,
                });
            },
        ).as('fetchProjectGroup');

        cy.mount(componentMount());

        cy.get('[data-cy="loading"]').should('be.visible');
        cy.wait('@fetchProjectGroup');

        cy.get('[data-cy="header"]').should('be.visible');
        cy.get('[data-cy="header"]').should('have.text', 'Project Group \'Test\'');
    });

    it('should show an error message', () => {
        cy.intercept(
            {
                method: 'GET',
                url: '/api/project-groups/e52744df-a140-4e72-b24c-18f534d0ae94',
            },
            (req) => {
                req.reply({
                    body: {
                        error: 'EXAMPLE',
                        description: 'ERROR',
                    },
                    statusCode: 400,
                });
            },
        ).as('fetchProjectGroup');

        cy.mount(componentMount());

        cy.wait('@fetchProjectGroup');
        cy.get('[data-cy="error"]').should('be.visible');
    });
});
