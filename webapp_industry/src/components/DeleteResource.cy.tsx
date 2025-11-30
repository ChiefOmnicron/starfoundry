import { MantineProvider } from "@mantine/core";
import { DeleteResource } from "./DeleteResource";

describe('Delete Resource Button General', () => {
    it('should show everything correctly', () => {
        cy.mount(
            <MantineProvider>
                <DeleteResource
                    resource={"Test"}
                    onConfirm={() => {}}
                />
            </MantineProvider>
        );

        cy.get('[data-cy="delete"]').click();
        cy.get('[data-cy="modalConfirmDelete"]').should('be.visible');
        cy.get('[data-cy="confirmDelete"]').should('be.disabled');

        cy.get('[data-cy="confirmDeleteText"').type('Test');
        cy.get('[data-cy="confirmDelete"]').should('be.enabled');
        cy.get('[data-cy="confirmDelete"]').click();
        cy.get('[data-cy="modalConfirmDelete"]').should('not.be.visible');
    });

    it('should accept lowercase', () => {
        cy.mount(
            <MantineProvider>
                <DeleteResource
                    resource={"TEST"}
                    onConfirm={() => {}}
                />
            </MantineProvider>
        );

        cy.get('[data-cy="delete"]').click();
        cy.get('[data-cy="modalConfirmDelete"]').should('be.visible');

        cy.get('[data-cy="confirmDeleteText"').type('test');
        cy.get('[data-cy="confirmDelete"]').should('be.enabled');
    });

    it('should not accept a wrong name', () => {
        cy.mount(
            <MantineProvider>
                <DeleteResource
                    resource={"Test"}
                    onConfirm={() => {}}
                />
            </MantineProvider>
        );

        cy.get('[data-cy="delete"]').click();
        cy.get('[data-cy="modalConfirmDelete"]').should('be.visible');

        cy.get('[data-cy="confirmDeleteText"').type('Test12354');
        cy.get('[data-cy="confirmDelete"]').should('be.disabled');
    });
});
