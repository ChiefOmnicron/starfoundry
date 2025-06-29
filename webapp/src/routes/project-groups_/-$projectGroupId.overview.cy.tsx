import { createRootRoute, createRouter, RouterProvider } from "@tanstack/react-router";
import { MantineProvider } from "@mantine/core";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { ProjectGroupOverview } from './$projectGroupId.overview';

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
        component: ProjectGroupOverview,
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

describe('Project Group Overview', () => {
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

        cy.intercept(
            {
                method: 'GET',
                url: '/api/project-groups/e52744df-a140-4e72-b24c-18f534d0ae94/can-write',
            },
            (req) => {
                req.reply({
                    statusCode: 204,
                });
            },
        ).as('canWriteProjectGroup');

        cy.mount(componentMount());

        cy.get('[data-cy="loading"]').should('be.visible');
        cy.wait('@fetchProjectGroup');
        cy.wait('@canWriteProjectGroup');

        cy.get('[data-cy="name"]').should('have.value', 'Test');
        cy.get('[data-cy="description"]').should('have.value', 'Some description');
    });

    it('should allow to create a group with only the name given', () => {
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
                    statusCode: 200,
                });
            },
        ).as('fetchProjectGroup');

        cy.intercept(
            {
                method: 'PUT',
                url: '/api/project-groups/e52744df-a140-4e72-b24c-18f534d0ae94',
            },
            (req) => {
                req.reply({
                    delay: 1000,
                    statusCode: 204,
                });
            },
        ).as('updateProjectGroup');

        cy.intercept(
            {
                method: 'GET',
                url: '/api/project-groups/e52744df-a140-4e72-b24c-18f534d0ae94/can-write',
            },
            (req) => {
                req.reply({
                    statusCode: 204,
                });
            },
        ).as('canWriteProjectGroup');

        cy.mount(componentMount());

        cy.wait('@fetchProjectGroup');
        cy.wait('@canWriteProjectGroup');

        cy.get('[data-cy="name"]').should('have.value', 'Test');
        cy.get('[data-cy="description"]').should('have.value', 'Some description');

        cy.get('[data-cy="name"]').clear().type('New Name');
        cy.get('[data-cy="description"]').clear();
        cy.get('[data-cy="name"]').should('have.value', 'New Name');
        cy.get('[data-cy="description"]').should('have.value', '');

        cy.get('[data-cy="save"]').click();
        cy.get('[data-cy="save"]').should('be.disabled');
        cy.get('[data-cy="save"]').should('have.attr', 'data-loading');
        cy.wait('@updateProjectGroup');

        cy.get('[data-cy="successfulUpdate"]').should('be.visible');
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
                        id: 'e52744df-a140-4e72-b24c-18f534d0ae94',
                        name: 'Test',
                        members: 1,
                        projects: 0,
                        is_owner: true,

                        description: 'Some description',
                    },
                    statusCode: 200,
                });
            },
        ).as('fetchProjectGroup');

        cy.intercept(
            {
                method: 'PUT',
                url: '/api/project-groups/e52744df-a140-4e72-b24c-18f534d0ae94',
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

        cy.intercept(
            {
                method: 'GET',
                url: '/api/project-groups/e52744df-a140-4e72-b24c-18f534d0ae94/can-write',
            },
            (req) => {
                req.reply({
                    statusCode: 204,
                });
            },
        ).as('canWriteProjectGroup');

        cy.mount(componentMount());

        cy.wait('@fetchProjectGroup');
        cy.wait('@canWriteProjectGroup');

        cy.get('[data-cy="name"]').should('have.value', 'Test');
        cy.get('[data-cy="description"]').should('have.value', 'Some description');

        cy.get('[data-cy="name"]').clear().type('New Name');
        cy.get('[data-cy="name"]').should('have.value', 'New Name');
        cy.get('[data-cy="description"]').clear();
        cy.get('[data-cy="description"]').should('have.value', '');

        cy.get('[data-cy="save"]').click();
        cy.get('[data-cy="save"]').should('be.disabled');
        cy.get('[data-cy="save"]').should('have.attr', 'data-loading');
        cy.wait('@updateProjectGroup');

        cy.get('[data-cy="errorUpdate"]').should('be.visible');
    });

    it('should not allow to update a group with only a description', () => {
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
                    statusCode: 200,
                });
            },
        ).as('fetchProjectGroup');

        cy.intercept(
            {
                method: 'GET',
                url: '/api/project-groups/e52744df-a140-4e72-b24c-18f534d0ae94/can-write',
            },
            (req) => {
                req.reply({
                    statusCode: 204,
                });
            },
        ).as('canWriteProjectGroup');

        cy.mount(componentMount());

        cy.wait('@fetchProjectGroup');
        cy.wait('@canWriteProjectGroup');
        cy.get('[data-cy="name"]').clear();
        cy.get('[data-cy="description"]').clear().type('Cool project');
        cy.get('[data-cy="save"]').should('be.disabled');
        cy.get('[data-cy="name"]').should('have.attr', 'data-error');
    });

    it('the user is allowed authorized to update the group', () => {
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
                    statusCode: 200,
                });
            },
        ).as('fetchProjectGroup');

        cy.intercept(
            {
                method: 'GET',
                url: '/api/project-groups/e52744df-a140-4e72-b24c-18f534d0ae94/can-write',
            },
            (req) => {
                req.reply({
                    statusCode: 204,
                });
            },
        ).as('canWriteProjectGroup');

        cy.mount(componentMount());

        cy.wait('@fetchProjectGroup');
        cy.wait('@canWriteProjectGroup');
        cy.get('[data-cy="name"]').should('be.enabled');
        cy.get('[data-cy="description"]').should('be.enabled');
        cy.get('[data-cy="save"]').should('exist');
        cy.get('[data-cy="danger-zone-header"]').should('exist');
        cy.get('[data-cy="danger-zone-card"]').should('exist');
    });

    it('the user is not authorized to update the group', () => {
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
                    statusCode: 200,
                });
            },
        ).as('fetchProjectGroup');

        cy.intercept(
            {
                method: 'GET',
                url: '/api/project-groups/e52744df-a140-4e72-b24c-18f534d0ae94/can-write',
            },
            (req) => {
                req.reply({
                    statusCode: 403,
                });
            },
        ).as('canWriteProjectGroup');

        cy.mount(componentMount());

        cy.wait('@fetchProjectGroup');
        cy.wait('@canWriteProjectGroup');
        cy.get('[data-cy="name"]').should('be.disabled');
        cy.get('[data-cy="description"]').should('be.disabled');
        cy.get('[data-cy="save"]').should('not.exist');
        cy.get('[data-cy="danger-zone-header"]').should('not.exist');
        cy.get('[data-cy="danger-zone-card"]').should('not.exist');
    });

    it('error while deleting', () => {
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
                    statusCode: 200,
                });
            },
        ).as('fetchProjectGroup');

        cy.intercept(
            {
                method: 'GET',
                url: '/api/project-groups/e52744df-a140-4e72-b24c-18f534d0ae94/can-write',
            },
            (req) => {
                req.reply({
                    statusCode: 204,
                });
            },
        ).as('canWriteProjectGroup');

        cy.intercept(
            {
                method: 'DELETE',
                url: '/api/project-groups/e52744df-a140-4e72-b24c-18f534d0ae94',
            },
            (req) => {
                req.reply({
                    statusCode: 400,
                });
            },
        ).as('deleteProjectGroup');

        cy.mount(componentMount());

        cy.wait('@fetchProjectGroup');
        cy.wait('@canWriteProjectGroup');
        cy.get('[data-cy="delete"]').click();
        cy.get('[data-cy="confirmDeleteText"]').type('Test');
        cy.get('[data-cy="confirmDelete"]').click();
        cy.wait('@deleteProjectGroup');
        cy.get('[data-cy="errorDelete"]').should('be.visible');
    });

    it('show the successfully created banner', () => {
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
                    statusCode: 200,
                });
            },
        ).as('fetchProjectGroup');

        cy.intercept(
            {
                method: 'GET',
                url: '/api/project-groups/e52744df-a140-4e72-b24c-18f534d0ae94/can-write',
            },
            (req) => {
                req.reply({
                    statusCode: 204,
                });
            },
        ).as('canWriteProjectGroup');


        const componentMount = () => {
            const routeTree = createRootRoute({
                component: ProjectGroupOverview,
                beforeLoad: ({ params, search }: { params: any, search: any }) => {
                    params.projectGroupId = 'e52744df-a140-4e72-b24c-18f534d0ae94';
                    search.created = true;
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

        cy.wait('@fetchProjectGroup');
        cy.wait('@canWriteProjectGroup');
        cy.get('[data-cy="createSuccessful"]').should('be.visible');
    });
});
