import { MantineProvider } from "@mantine/core";
import { ResolveStructure } from "./ResolveStructure";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

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

describe('ResolveStructure Input', () => {
    it('should show the loading spinner', () => {
        cy.intercept(
            {
                method: 'GET',
                url: '/api/auth/token',
            },
            (req) => {
                req.reply({
                    body: {
                        access_token: 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiYWRtaW4iOnRydWUsImlhdCI6MTUxNjIzOTAyMn0.KMUFsIDTnFmyG3nMiGM6H9FNFUROf3wh7SmqJp-QV30'
                    },
                    statusCode: 200,
                });
            },
        ).as('AuthToken');
        cy.intercept(
            {
                method: 'GET',
                url: '/api/eve-gateway/structures/1000000000000',
            },
            (req) => {
                req.reply({
                    body: {},
                    delay: 1000,
                    statusCode: 200,
                });
            },
        ).as('ResolveStructure');

        cy.mount(
            <MantineProvider>
                <QueryClientProvider client={queryClient}>
                    <ResolveStructure
                        onError={() => {}}
                        onSuccess={() => {}}
                        onLoad={() => {}}
                    />
                </QueryClientProvider>
            </MantineProvider>
        );

        cy.get('[data-cy="structureId"]').type('Some Character > <url=showinfo:35834//1000000000000>SomeSystem - Some Structure</url>');
        cy.get('[data-cy="resolveStructure"').click();

        cy.get('[data-cy="resolveStructure"').should('be.disabled');
        cy.get('[data-cy="resolveStructure"').should('have.attr', 'data-loading');

        cy.wait('@AuthToken');
        cy.wait('@ResolveStructure');

        cy.get('[data-cy="resolveStructure"').should('not.be.disabled');
        cy.get('[data-cy="resolveStructure"').should('not.have.attr', 'data-loading');
    });
});
