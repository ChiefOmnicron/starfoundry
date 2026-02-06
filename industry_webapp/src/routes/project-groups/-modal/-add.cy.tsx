import { createRootRoute, createRouter, RouterProvider } from "@tanstack/react-router";
import { MantineProvider } from "@mantine/core";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { AddProjectGroup } from '@/routes/project-groups/-modal/add';

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
        component: () => <AddProjectGroup close={() => {}} />,
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
    it('should allow to create a group with only the name given', () => {
        cy.intercept(
            {
                method: 'POST',
                url: '/api/project-groups',
            },
            (req) => {
                req.reply({
                    body: {
                        id: 'e52744df-a140-4e72-b24c-18f534d0ae94',
                    },
                    delay: 1000,
                    statusCode: 201,
                });
            },
        ).as('AddProjectGroup');

        cy.mount(componentMount());

        cy.get('[data-cy="name"]').type('Cool project');
        cy.get('[data-cy="create"').click();
        cy.get('[data-cy="create"').should('be.disabled');
        cy.get('[data-cy="create"').should('have.attr', 'data-loading');
        cy.wait('@AddProjectGroup');
    });

    it('should show an error notification', () => {
        cy.intercept(
            {
                method: 'POST',
                url: '/api/project-groups',
            },
            (req) => {
                req.reply({
                    body: {
                        error: 'DESERIALIZE',
                        description: 'Just some test error'
                    },
                    delay: 1000,
                    statusCode: 400,
                });
            },
        ).as('updateProjectGroup');

        cy.mount(componentMount());

        cy.get('[data-cy="name"]').type('Test')
        cy.get('[data-cy="name"]').should('have.value', 'Test');
        cy.get('[data-cy="description"]').type('Some description');
        cy.get('[data-cy="description"]').should('have.value', 'Some description');

        cy.get('[data-cy="create"]').click();
        cy.get('[data-cy="create"]').should('be.disabled');
        cy.get('[data-cy="create"]').should('have.attr', 'data-loading');
        cy.wait('@updateProjectGroup');

        cy.get('[data-cy="errorCreate"]').should('be.visible');
    });

    it('should not allow to create a group with only a description', () => {
        cy.mount(componentMount());

        cy.get('[data-cy="description"]').type('Cool project');
        cy.get('[data-cy="create"').click();
        cy.get('[data-cy="name"]').should('have.attr', 'data-error');
    });
});
